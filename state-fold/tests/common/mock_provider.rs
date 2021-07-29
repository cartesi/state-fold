use offchain_core::types::Block;
use offchain_utils::offchain_core;

use state_fold::{
    delegate_access::Result, DelegateAccess, FoldAccess, SyncAccess,
};

use async_trait::async_trait;
use offchain_core::ethers::providers::{Middleware, MockProvider, Provider};
use offchain_core::ethers::types::{
    Address, BlockId, BlockNumber, Bloom, Bytes, TransactionRequest, H256,
    U256, U64,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct MockBlock {
    pub number: U64,
    pub hash: H256,
    pub parent_hash: H256,
    pub branch: String,
}

impl Into<Block> for MockBlock {
    fn into(self) -> Block {
        Block {
            hash: self.hash,
            number: self.number,
            parent_hash: self.parent_hash,
            timestamp: U256::zero(),
            logs_bloom: Bloom::zero(),
        }
    }
}

#[derive(Debug)]
pub struct MockAccess {
    chain: Mutex<HashMap<H256, MockBlock>>,
    block_count: Mutex<U64>,
    latest_block: Mutex<H256>,

    mock_provider: MockProvider,
    provider: Arc<Provider<MockProvider>>,
}

impl MockAccess {
    pub async fn new(initial_block_count: u64, initial_branch: &str) -> Self {
        assert!(initial_block_count > 0);

        let (p, m) = Provider::mocked();
        let latest_block = H256::zero();

        let this = Self {
            chain: Mutex::new(HashMap::new()),
            block_count: Mutex::new(U64::from(0)),
            latest_block: Mutex::new(latest_block),

            mock_provider: m,
            provider: Arc::new(p),
        };

        this.chain.lock().await.insert(
            latest_block,
            MockBlock {
                number: *this.block_count.lock().await,
                hash: latest_block,
                parent_hash: latest_block,
                branch: initial_branch.to_string(),
            },
        );

        let mut previous_hash = *this.latest_block.lock().await;
        for _ in 0..initial_block_count {
            previous_hash =
                this.add_block(previous_hash, initial_branch).await.unwrap();
        }

        this
    }

    pub async fn get_hash_of(&self, number: U64, branch: &str) -> Option<H256> {
        // Iterate over everything.
        for (hash, block) in &*self.chain.lock().await {
            if block.number == number && block.branch == branch {
                return Some(hash.clone());
            }
        }
        None
    }

    pub async fn add_block(
        &self,
        parent_hash: H256,
        branch: &str,
    ) -> Option<H256> {
        let new_number =
            self.chain.lock().await.get(&parent_hash)?.number + U64::from(1);
        let new_hash = self.new_hash().await;
        let new_block = MockBlock {
            number: new_number,
            hash: new_hash,
            parent_hash,
            branch: branch.to_string(),
        };
        self.chain.lock().await.insert(new_hash, new_block);
        *self.latest_block.lock().await = new_hash;
        Some(new_hash)
    }

    pub async fn get_block(&self, hash: H256) -> Option<MockBlock> {
        self.chain.lock().await.get(&hash).map(|x| x.clone())
    }

    pub async fn get_block_with_number(
        &self,
        number: U64,
    ) -> Option<MockBlock> {
        self.get_block_with_number_from(number, *self.latest_block.lock().await)
            .await
    }

    pub async fn get_block_with_number_from(
        &self,
        number: U64,
        tip: H256,
    ) -> Option<MockBlock> {
        let mut current_hash = tip;

        loop {
            match self.chain.lock().await.get(&current_hash) {
                Some(block) => {
                    if block.number == number {
                        return Some(block.clone());
                    } else if block.number == 0.into() {
                        return None;
                    } else {
                        current_hash = block.parent_hash;
                    }
                }
                None => break,
            }
        }

        None
    }

    pub async fn get_latest_block(&self) -> Option<MockBlock> {
        self.chain
            .lock()
            .await
            .get(&*self.latest_block.lock().await)
            .map(|x| x.clone())
    }

    pub async fn get_latest_hash(&self) -> H256 {
        *self.latest_block.lock().await
    }

    async fn new_hash(&self) -> H256 {
        *self.block_count.lock().await += U64::from(1);
        H256::from_low_u64_be(self.block_count.lock().await.as_u64())
    }
}

#[async_trait]
impl DelegateAccess for MockAccess {
    async fn get_block<T: Into<BlockId> + Send + Sync>(
        &self,
        block: T,
    ) -> Result<Block, Self::InnerMiddleware> {
        match block.into() {
            BlockId::Hash(h) => Ok(self.get_block(h).await.unwrap().into()),
            BlockId::Number(BlockNumber::Number(n)) => {
                Ok(self.get_block_with_number(n).await.unwrap().into())
            }
            x => panic!("get_block not number {:?}", x),
        }
    }

    async fn get_current_block_number(
        &self,
    ) -> Result<U64, Self::InnerMiddleware> {
        Ok(self.get_latest_block().await.unwrap().number)
    }
}

#[async_trait]
impl SyncAccess for MockAccess {
    type SyncAccessMiddleware = MockMiddleware;

    async fn build_sync_contract<T: Into<Address> + Send + Sync, C>(
        &self,
        address: T,
        block_number: U64,
        init: impl FnOnce(T, Arc<Self::SyncAccessMiddleware>) -> C
            + Send
            + Sync
            + 'static,
    ) -> C {
        let mock_block =
            self.get_block_with_number(block_number).await.unwrap();
        let middleware = Arc::new(MockMiddleware {
            inner: self.provider.clone(),
            block_hash: mock_block.hash,
            block_number: mock_block.number,
            branch: mock_block.branch.clone(),
        });

        init(address, middleware)
    }
}

#[async_trait]
impl FoldAccess for MockAccess {
    type InnerMiddleware = Provider<MockProvider>;
    type FoldAccessMiddleware = MockMiddleware;

    async fn build_fold_contract<T: Into<Address> + Send + Sync, C>(
        &self,
        address: T,
        block_hash: H256,
        init: impl FnOnce(T, Arc<Self::FoldAccessMiddleware>) -> C
            + Send
            + Sync
            + 'static,
    ) -> C {
        let mock_block = self.get_block(block_hash).await.unwrap();
        let middleware = Arc::new(MockMiddleware {
            inner: self.provider.clone(),
            block_hash: mock_block.hash,
            block_number: mock_block.number,
            branch: mock_block.branch.clone(),
        });

        init(address, middleware)
    }
}

#[derive(Debug)]
pub struct MockMiddleware {
    inner: Arc<Provider<MockProvider>>,
    block_hash: H256,
    block_number: U64,
    branch: String,
}

#[async_trait]
impl Middleware for MockMiddleware {
    type Error =
        state_fold::delegate_access::AccessMiddlewareError<Self::Inner>;
    type Provider = MockProvider;
    type Inner = Provider<MockProvider>;

    fn inner(&self) -> &Provider<MockProvider> {
        &self.inner
    }

    async fn call(
        &self,
        _: &TransactionRequest,
        _: Option<BlockId>,
    ) -> std::result::Result<Bytes, Self::Error> {
        // Silly way to pass parameters to delegate
        let x = serde_json::ser::to_vec(&(
            self.block_hash,
            self.block_number,
            self.branch.clone(),
        ))
        .unwrap();
        Ok(x.into())
    }
}
