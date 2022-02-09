use crate::block_archive::{self, BlockArchive};

use state_fold_types::{BlockStreamItem, BlocksSince};

use offchain_core::ethers::providers::{Middleware, PubsubClient};
use offchain_core::types::Block;
use offchain_utils::offchain_core;

use std::sync::Arc;
use tokio::sync::{oneshot, watch};
use tokio_stream::{Stream, StreamExt};

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum BlockSubscriberError<
    M: offchain_core::ethers::providers::Middleware + 'static,
> {
    #[snafu(display("Ethers provider error: {}", source))]
    EthersProviderError { source: M::Error },

    #[snafu(display("Got incomplete block"))]
    BlockIncomplete { err: String },

    #[snafu(display("New block subscriber timeout: {}", source))]
    NewBlockSubscriberTimeout { source: std::io::Error },

    #[snafu(display("Ethers subscription dropped"))]
    EthersSubscriptionDropped {},
}

pub type Result<T, M> = std::result::Result<T, BlockSubscriberError<M>>;

#[derive(Debug, Snafu)]
pub enum SubscriptionError {
    #[snafu(display("Subscriber dropped: {}", source))]
    SubscriptionDropped {
        source: tokio::sync::watch::error::RecvError,
    },

    #[snafu(display("Error while accessing block archive: {}", source))]
    ArchiveError {
        source: crate::block_archive::BlockArchiveError,
    },
}
pub type SubscriptionResult<T> = std::result::Result<T, SubscriptionError>;

pub struct BlockSubscriber<M: Middleware + 'static> {
    pub handle: tokio::task::JoinHandle<Result<(), M>>,
    pub block_archive: Arc<BlockArchive<M>>,

    new_block_alarm: watch::Receiver<()>,
    _kill_switch: oneshot::Sender<()>,
}

impl<M: Middleware + 'static> BlockSubscriber<M>
where
    <M as Middleware>::Provider: PubsubClient,
{
    pub async fn start(
        middleware: Arc<M>,
        subscriber_timeout: std::time::Duration,
        max_depth: usize,
    ) -> crate::block_archive::Result<Self> {
        let archive = Arc::new(BlockArchive::new(middleware.clone(), max_depth).await?);

        let (kill_tx, kill_rx) = oneshot::channel();
        let (new_block_tx, new_block_alarm) = watch::channel(());

        let block_archive = archive.clone();

        // Create background task and detach it.
        let handle = tokio::spawn(async move {
            // Create future of `background_process` main loop. This
            // future will run against the kill_switch.
            let task = background_process(
                middleware,
                archive,
                new_block_tx,
                subscriber_timeout,
            );
            tokio::pin!(task);

            tokio::select! {
                res = &mut task => {
                    return res
                },

                _ = kill_rx => {
                    return Ok(())
                }
            }
        });

        Ok(Self {
            handle,
            block_archive,
            new_block_alarm,
            _kill_switch: kill_tx,
        })
    }

    pub async fn subscribe_new_blocks_at_depth(
        &self,
        depth: usize,
    ) -> block_archive::Result<
        impl Stream<Item = SubscriptionResult<BlockStreamItem>> + Unpin,
    > {
        let archive = self.block_archive.clone();
        let mut alarm = self.new_block_alarm.clone();

        let mut previous = archive.block_at_depth(depth).await?;

        Ok(Box::pin(async_stream::try_stream! {
            while let () = alarm.changed().await.context(SubscriptionDropped)? {
                let diff = archive
                    .blocks_since(depth, &previous)
                    .await
                    .context(ArchiveError)?;

                match diff {
                    BlocksSince::Normal(blocks) => {
                        if let Some(p) = blocks.last() {
                            previous = p.clone();
                            for b in blocks {
                                yield BlockStreamItem::NewBlock(b);
                            }
                        }
                    }

                    BlocksSince::Reorg(blocks) => {
                        yield BlockStreamItem::Reorg(blocks);
                    }
                }
            }
        }))
    }
}

async fn background_process<M: Middleware + 'static>(
    middleware: Arc<M>,
    block_archive: Arc<BlockArchive<M>>,
    new_block_alarm: watch::Sender<()>,
    subscriber_timeout: std::time::Duration,
) -> Result<(), M>
where
    <M as Middleware>::Provider: PubsubClient,
{
    loop {
        let subscription = middleware
            .subscribe_blocks()
            .await
            .context(EthersProviderError)
            .map(|subscription| {
                Box::pin(subscription.timeout(subscriber_timeout).map(|x| {
                    let block_header = x
                        .map_err(|e| e.into())
                        .context(NewBlockSubscriberTimeout)?;

                    let block = block_header
                        .try_into()
                        .map_err(|err| BlockIncomplete { err }.build())?;

                    Ok(block)
                }))
            })?;

        if let Err(_) = listen_and_broadcast(
            block_archive.clone(),
            &new_block_alarm,
            subscription,
        )
        .await
        {
            // TODO: Warn error
        }
    }
}

async fn listen_and_broadcast<M: Middleware + 'static>(
    block_archive: Arc<BlockArchive<M>>,
    new_block_alarm: &watch::Sender<()>,
    mut subscription: impl Stream<Item = Result<Block, M>> + Send + Unpin,
) -> Result<(), M> {
    // Listen to new blocks and notify subscribers.
    loop {
        // Block on waiting for new block.
        let new_head = subscription
            .next()
            .await
            .ok_or(snafu::NoneError)
            .context(EthersSubscriptionDropped)??;

        // Insert in archive
        let _ = block_archive.update_latest_block(new_head).await;

        // Send new block to subscribers.
        if let Err(_) = new_block_alarm.send(()) {
            // TODO: warn there are no subscribers.
        }
    }
}
