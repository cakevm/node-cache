use crate::error::{alloy_error, eyre_error};
use alloy_primitives::{Address, Bytes, B256, U256, U64};
use alloy_provider::network::Ethereum;
use alloy_provider::Provider;
use alloy_rpc_types_eth::state::StateOverride;
use alloy_rpc_types_eth::{
    BlockId, BlockNumberOrTag, BlockOverrides, EIP1186AccountProofResponse, Filter, Log, SyncStatus, TransactionRequest,
};
use alloy_serde::JsonStorageKey;
use alloy_transport::Transport;
use jsonrpsee::core::RpcResult;
use node_cache_recorder::Recorder;
use reth_rpc::eth::EthereumEthApiTypes;
use reth_rpc_api::eth::RpcBlock;
use reth_rpc_api::EngineEthApiServer;
use reth_rpc_eth_api::EthApiTypes;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct NodeCacheEngineEthApi<T, P, R> {
    provider: Option<P>,
    _t: std::marker::PhantomData<T>,
    recorder: Arc<R>,
}

impl<T, P, R> NodeCacheEngineEthApi<T, P, R>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Send + Sync + Clone + 'static,
    R: Recorder,
{
    pub const fn new(provider: Option<P>, recorder: Arc<R>) -> Self {
        Self { provider, _t: std::marker::PhantomData, recorder }
    }
}

#[async_trait::async_trait]
impl<T, P, R> EngineEthApiServer<RpcBlock<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>> for NodeCacheEngineEthApi<T, P, R>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Send + Sync + Clone + 'static,
    R: Recorder + Sync + Send + 'static,
{
    fn syncing(&self) -> RpcResult<SyncStatus> {
        todo!()
    }

    async fn chain_id(&self) -> RpcResult<Option<U64>> {
        todo!()
    }

    fn block_number(&self) -> RpcResult<U256> {
        Ok(U256::from(1))
    }

    async fn call(
        &self,
        _request: TransactionRequest,
        _block_id: Option<BlockId>,
        _state_overrides: Option<StateOverride>,
        _block_overrides: Option<Box<BlockOverrides>>,
    ) -> RpcResult<Bytes> {
        todo!()
    }

    async fn get_code(&self, _address: Address, _block_id: Option<BlockId>) -> RpcResult<Bytes> {
        todo!()
    }

    async fn block_by_hash(
        &self,
        _hash: B256,
        _full: bool,
    ) -> RpcResult<Option<RpcBlock<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>>> {
        todo!()
    }

    async fn block_by_number(
        &self,
        number: BlockNumberOrTag,
        full: bool,
    ) -> RpcResult<Option<RpcBlock<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>>> {
        let key = format!("{}{}{}", "block_by_number", number, full);
        if let Some(provider) = self.provider.as_ref() {
            let block = provider.get_block_by_number(number, full.into()).await.map_err(alloy_error)?;
            let rpc_block: Option<RpcBlock<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>> = block;
            if let Some(ref block) = rpc_block {
                self.recorder.record(&key, block).await.map_err(eyre_error)?;
            }
            return Ok(rpc_block);
        }

        let ret = self.recorder.get::<RpcBlock<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>>(&key).await.map_err(eyre_error)?;
        Ok(ret)
    }

    async fn send_raw_transaction(&self, _bytes: Bytes) -> RpcResult<B256> {
        todo!()
    }

    async fn logs(&self, _filter: Filter) -> RpcResult<Vec<Log>> {
        todo!()
    }

    async fn get_proof(
        &self,
        _address: Address,
        _keys: Vec<JsonStorageKey>,
        _block_number: Option<BlockId>,
    ) -> RpcResult<EIP1186AccountProofResponse> {
        todo!()
    }
}
