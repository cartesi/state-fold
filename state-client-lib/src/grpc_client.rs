use crate::{error::*, BlockServer, StateServer};

use ethers::core::types::H256;
use offchain_core::ethers;
use offchain_core::types::Block;
use offchain_utils::offchain_core;
use state_fold_types::{
    BlockState, BlockStreamItem, BlocksSince, QueryBlock, StateStreamItem,
    StatesSince,
};

use state_fold_server::state_fold_client::StateFoldClient;
use state_fold_server::{
    InitialState, QueryBlockRequest, QueryBlocksSinceRequest,
    QueryStateRequest, QueryStatesSinceRequest, SubscribeNewBlocksRequest,
    SubscribeNewStatesRequest,
};
use state_server_common::state_fold_server;
use tonic::{transport::Channel, Request};

use serde;
use snafu::ResultExt;

use async_trait::async_trait;
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};

struct GrpcStateFoldClient<I, S> {
    client: StateFoldClient<Channel>,
    __marker1: std::marker::PhantomData<I>,
    __marker2: std::marker::PhantomData<S>,
}

#[async_trait]
impl<I, S> BlockServer for GrpcStateFoldClient<I, S>
where
    I: Send + Sync,
    S: Send + Sync,
{
    async fn query_block(
        &self,
        query_block: impl Into<QueryBlock> + Send + 'static,
    ) -> Result<Block> {
        let mut client = self.client.clone();

        let query_block: QueryBlock = query_block.into();
        let request = Request::new(QueryBlockRequest {
            query_block: Some(query_block.into()),
        });

        let block = client
            .query_block(request)
            .await
            .context(TonicError {
                context: "`get_block` request",
            })?
            .into_inner()
            .try_into()
            .context(MessageConversion {
                context: "`get_block`".to_owned(),
            })?;

        Ok(block)
    }

    async fn query_blocks_since(
        &self,
        previous_block_hash: H256,
        depth: usize,
    ) -> Result<BlocksSince> {
        let mut client = self.client.clone();

        let request = Request::new(QueryBlocksSinceRequest {
            previous_block: Some(previous_block_hash.into()),
            depth: depth as u64,
        });

        let diff = client
            .query_blocks_since(request)
            .await
            .context(TonicError {
                context: "`get_block_diff` request",
            })?
            .into_inner()
            .try_into()
            .context(MessageConversion {
                context: "`get_block_diff`".to_owned(),
            })?;

        Ok(diff)
    }

    async fn subscribe_blocks(
        &self,
        confirmations: usize,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<BlockStreamItem>>>>> {
        let mut client = self.client.clone();

        let request = Request::new(SubscribeNewBlocksRequest {
            confirmations: confirmations as u64,
        });

        let stream = client
            .subscribe_new_blocks(request)
            .await
            .context(TonicError {
                context: "`subscribe_blocks` request",
            })?
            .into_inner();

        let stream = stream.map(|b| -> Result<BlockStreamItem> {
            b.context(TonicError {
                context: "`subscribe_blocks` stream item conversion",
            })?
            .try_into()
            .context(MessageConversion {
                context: "`subscribe_blocks` stream item conversion",
            })
        });

        Ok(Box::pin(stream))
    }
}

#[async_trait]
impl<I, S> StateServer for GrpcStateFoldClient<I, S>
where
    I: serde::Serialize + Send + Sync,
    S: serde::de::DeserializeOwned + Send + Sync,
{
    type InitialState = I;
    type State = S;

    async fn query_state(
        &self,
        initial_state: Self::InitialState,
        query_block: impl Into<QueryBlock> + Send + 'static,
    ) -> Result<BlockState<Self::State>> {
        let mut client = self.client.clone();

        let initial_state_json = InitialState {
            json_data: serde_json::to_string(&initial_state)
                .context(SerializeError)?,
        };

        let query_block: QueryBlock = query_block.into();

        let request = Request::new(QueryStateRequest {
            initial_state: Some(initial_state_json),
            query_block: Some(query_block.into()),
        });

        let state = client
            .query_state(request)
            .await
            .context(TonicError {
                context: "`get_state` request",
            })?
            .into_inner()
            .try_into()
            .context(StateConversion {
                context: "`get_state`".to_owned(),
            })?;

        Ok(state)
    }

    async fn query_states_since(
        &self,
        initial_state: Self::InitialState,
        previous_block_hash: H256,
        depth: usize,
    ) -> Result<StatesSince<Self::State>> {
        let mut client = self.client.clone();

        let initial_state_json = InitialState {
            json_data: serde_json::to_string(&initial_state)
                .context(SerializeError)?,
        };

        let request = Request::new(QueryStatesSinceRequest {
            initial_state: Some(initial_state_json),
            previous_block: Some(previous_block_hash.into()),
            depth: depth as u64,
        });

        let diff = client
            .query_states_since(request)
            .await
            .context(TonicError {
                context: "`get_state_diff` request",
            })?
            .into_inner()
            .try_into()
            .context(StateConversion {
                context: "`get_state_diff`".to_owned(),
            })?;

        Ok(diff)
    }

    async fn subscribe_states(
        &self,
        initial_state: Self::InitialState,
        confirmations: usize,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StateStreamItem<Self::State>>>>>>
    {
        let mut client = self.client.clone();

        let initial_state_json = InitialState {
            json_data: serde_json::to_string(&initial_state)
                .context(SerializeError)?,
        };

        let request = Request::new(SubscribeNewStatesRequest {
            initial_state: Some(initial_state_json),
            confirmations: confirmations as u64,
        });

        let stream = client
            .subscribe_new_states(request)
            .await
            .context(TonicError {
                context: "`subscribe_blocks` request",
            })?
            .into_inner();

        let stream = stream.map(|s| -> Result<StateStreamItem<Self::State>> {
            s.context(TonicError {
                context: "`subscribe_blocks` stream item conversion",
            })?
            .try_into()
            .context(StateConversion {
                context: "`subscribe_blocks` stream item conversion",
            })
        });

        Ok(Box::pin(stream))
    }
}