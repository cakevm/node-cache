use crate::helper::base::{build_inner, ApiInner};
use crate::helper::error::{alloy_error, eyre_error};
use alloy_dyn_abi::TypedData;
use alloy_primitives::utils::parse_units;
use alloy_primitives::{Address, Bytes, B256, B64, U256, U64};
use alloy_provider::network::Ethereum;
use alloy_provider::Provider;
use alloy_rpc_types_eth::simulate::{SimulatePayload, SimulatedBlock};
use alloy_rpc_types_eth::state::StateOverride;
use alloy_rpc_types_eth::{
    AccessListResult, Account, BlockId, BlockNumberOrTag, BlockOverrides, Bundle, EIP1186AccountProofResponse, EthCallResponse, FeeHistory,
    Header, Index, StateContext, SyncStatus, TransactionRequest, Work,
};
use alloy_serde::JsonStorageKey;
use alloy_transport::Transport;
use jsonrpsee::core::RpcResult;
use jsonrpsee::tokio;
use jsonrpsee::tokio::runtime::Handle;
use node_cache_recorder::Recorder;
use reth_rpc::eth::EthereumEthApiTypes;
use reth_rpc_eth_api::{EthApiServer, EthApiTypes, RpcBlock, RpcReceipt, RpcTransaction};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct NodeCacheEthApi<T, P, R>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Send + Sync + Clone + 'static,
    R: Recorder,
{
    inner: ApiInner<T, P, R>,
}

impl<T, P, R> NodeCacheEthApi<T, P, R>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Send + Sync + Clone + 'static,
    R: Recorder,
{
    pub fn new(provider: Option<P>, recorder: Arc<R>) -> Self {
        Self { inner: build_inner(provider, recorder) }
    }
}

#[async_trait::async_trait]
impl<T, P, R>
    EthApiServer<
        RpcTransaction<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>,
        RpcBlock<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>,
        RpcReceipt<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>,
    > for NodeCacheEthApi<T, P, R>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Send + Sync + Clone + 'static,
    R: Recorder + Sync + Send + 'static,
{
    async fn protocol_version(&self) -> RpcResult<U64> {
        todo!()
    }

    fn syncing(&self) -> RpcResult<SyncStatus> {
        todo!()
    }

    async fn author(&self) -> RpcResult<Address> {
        todo!()
    }

    fn accounts(&self) -> RpcResult<Vec<Address>> {
        todo!()
    }

    fn block_number(&self) -> RpcResult<U256> {
        if let Some(provider) = self.inner.provider.as_ref() {
            let block_number =
                tokio::task::block_in_place(move || Handle::current().block_on(provider.get_block_number())).map_err(alloy_error)?;

            return Ok(U256::from(block_number));
        }
        panic!("Provider required")
    }

    async fn chain_id(&self) -> RpcResult<Option<U64>> {
        Ok(Some(U64::from(1)))
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
        // cache
        if let Some(ret) = self.inner.recorder.get(&key).await.map_err(eyre_error)? {
            return Ok(Some(ret));
        }
        // real provider
        if let Some(provider) = self.inner.provider.as_ref() {
            let result = provider.get_block_by_number(number, full.into()).await.map_err(alloy_error)?;
            if let Some(ref block) = result {
                self.inner.recorder.record(&key, block).await.map_err(eyre_error)?;
            }
            return Ok(result);
        }
        Ok(None)
    }

    async fn block_transaction_count_by_hash(&self, _hash: B256) -> RpcResult<Option<U256>> {
        todo!()
    }

    async fn block_transaction_count_by_number(&self, _number: BlockNumberOrTag) -> RpcResult<Option<U256>> {
        todo!()
    }

    async fn block_uncles_count_by_hash(&self, _hash: B256) -> RpcResult<Option<U256>> {
        todo!()
    }

    async fn block_uncles_count_by_number(&self, _number: BlockNumberOrTag) -> RpcResult<Option<U256>> {
        todo!()
    }

    async fn block_receipts(
        &self,
        _block_id: BlockId,
    ) -> RpcResult<Option<Vec<RpcReceipt<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>>>> {
        todo!()
    }

    async fn uncle_by_block_hash_and_index(
        &self,
        _hash: B256,
        _index: Index,
    ) -> RpcResult<Option<RpcBlock<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>>> {
        todo!()
    }

    async fn uncle_by_block_number_and_index(
        &self,
        _number: BlockNumberOrTag,
        _index: Index,
    ) -> RpcResult<Option<RpcBlock<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>>> {
        todo!()
    }

    async fn raw_transaction_by_hash(&self, _hash: B256) -> RpcResult<Option<Bytes>> {
        todo!()
    }

    async fn transaction_by_hash(
        &self,
        hash: B256,
    ) -> RpcResult<Option<RpcTransaction<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>>> {
        let key = format!("{}{}", "transaction_by_hash", hash);
        // cache
        if let Some(ret) = self.inner.recorder.get(&key).await.map_err(eyre_error)? {
            return Ok(ret);
        }
        // real provider
        if let Some(provider) = self.inner.provider.as_ref() {
            let result = provider.get_transaction_by_hash(hash).await.map_err(alloy_error)?;
            self.inner.recorder.record(&key, &result).await.map_err(eyre_error)?;
            return Ok(result);
        }
        Ok(None)
    }

    async fn raw_transaction_by_block_hash_and_index(&self, _hash: B256, _index: Index) -> RpcResult<Option<Bytes>> {
        todo!()
    }

    async fn transaction_by_block_hash_and_index(
        &self,
        _hash: B256,
        _index: Index,
    ) -> RpcResult<Option<RpcTransaction<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>>> {
        todo!()
    }

    async fn raw_transaction_by_block_number_and_index(&self, _number: BlockNumberOrTag, _index: Index) -> RpcResult<Option<Bytes>> {
        todo!()
    }

    async fn transaction_by_block_number_and_index(
        &self,
        _number: BlockNumberOrTag,
        _index: Index,
    ) -> RpcResult<Option<RpcTransaction<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>>> {
        todo!()
    }

    async fn transaction_by_sender_and_nonce(
        &self,
        _address: Address,
        _nonce: U64,
    ) -> RpcResult<Option<RpcTransaction<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>>> {
        todo!()
    }

    async fn transaction_receipt(&self, _hash: B256) -> RpcResult<Option<RpcReceipt<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>>> {
        todo!()
    }

    async fn balance(&self, address: Address, block_number: Option<BlockId>) -> RpcResult<U256> {
        let key = format!("{}{}{:?}", "get_balance", address, block_number);
        // cache
        if let Some(ret) = self.inner.recorder.get(&key).await.map_err(eyre_error)? {
            return Ok(ret);
        }
        // real provider
        if let Some(provider) = self.inner.provider.as_ref() {
            let result = match block_number {
                Some(block_id) => provider.get_balance(address).block_id(block_id).await.map_err(alloy_error)?,
                None => provider.get_balance(address).await.map_err(alloy_error)?,
            };
            self.inner.recorder.record(&key, &result).await.map_err(eyre_error)?;
            return Ok(result);
        }
        Ok(U256::ZERO)
    }

    async fn storage_at(&self, address: Address, index: JsonStorageKey, block_number: Option<BlockId>) -> RpcResult<B256> {
        let key = format!("{}{}{}{:?}", "storage_at", address, index, block_number);
        // cache
        if let Some(ret) = self.inner.recorder.get(&key).await.map_err(eyre_error)? {
            return Ok(ret);
        }
        // real provider
        if let Some(provider) = self.inner.provider.as_ref() {
            let result = match block_number {
                Some(block_id) => provider
                    .get_storage_at(address, U256::from_be_slice(index.as_b256().as_slice()))
                    .block_id(block_id)
                    .await
                    .map_err(alloy_error)?,
                None => provider.get_storage_at(address, U256::from_be_slice(index.as_b256().as_slice())).await.map_err(alloy_error)?,
            };
            self.inner.recorder.record(&key, &result).await.map_err(eyre_error)?;
            return Ok(B256::from(result));
        }
        Ok(B256::ZERO)
    }

    async fn transaction_count(&self, address: Address, block_number: Option<BlockId>) -> RpcResult<U256> {
        let key = format!("{}{}{:?}", "transaction_count", address, block_number);
        // cache
        if let Some(ret) = self.inner.recorder.get(&key).await.map_err(eyre_error)? {
            return Ok(ret);
        }
        // real provider
        if let Some(provider) = self.inner.provider.as_ref() {
            let result = match block_number {
                Some(block_id) => provider.get_transaction_count(address).block_id(block_id).await.map_err(alloy_error)?,
                None => provider.get_transaction_count(address).await.map_err(alloy_error)?,
            };
            self.inner.recorder.record(&key, &result).await.map_err(eyre_error)?;
            return Ok(U256::from(result));
        }
        Ok(U256::ZERO)
    }

    async fn get_code(&self, address: Address, block_number: Option<BlockId>) -> RpcResult<Bytes> {
        let key = format!("{}{}{:?}", "get_code", address, block_number);
        // cache
        if let Some(ret) = self.inner.recorder.get(&key).await.map_err(eyre_error)? {
            return Ok(ret);
        }
        // real provider
        if let Some(provider) = self.inner.provider.as_ref() {
            let result = match block_number {
                Some(block_id) => provider.get_code_at(address).block_id(block_id).await.map_err(alloy_error)?,
                None => provider.get_code_at(address).await.map_err(alloy_error)?,
            };
            self.inner.recorder.record(&key, &result).await.map_err(eyre_error)?;
            return Ok(result);
        }
        Ok(Bytes::new())
    }

    async fn header_by_number(&self, _hash: BlockNumberOrTag) -> RpcResult<Option<Header>> {
        todo!()
    }

    async fn header_by_hash(&self, _hash: B256) -> RpcResult<Option<Header>> {
        todo!()
    }

    async fn simulate_v1(
        &self,
        _opts: SimulatePayload,
        _block_number: Option<BlockId>,
    ) -> RpcResult<Vec<SimulatedBlock<RpcBlock<<EthereumEthApiTypes as EthApiTypes>::NetworkTypes>>>> {
        todo!()
    }

    async fn call(
        &self,
        _request: TransactionRequest,
        _block_number: Option<BlockId>,
        _state_overrides: Option<StateOverride>,
        _block_overrides: Option<Box<BlockOverrides>>,
    ) -> RpcResult<Bytes> {
        todo!()
    }

    async fn call_many(
        &self,
        _bundle: Bundle,
        _state_context: Option<StateContext>,
        _state_override: Option<StateOverride>,
    ) -> RpcResult<Vec<EthCallResponse>> {
        todo!()
    }

    async fn create_access_list(&self, _request: TransactionRequest, _block_number: Option<BlockId>) -> RpcResult<AccessListResult> {
        todo!()
    }

    async fn estimate_gas(
        &self,
        _request: TransactionRequest,
        _block_number: Option<BlockId>,
        _state_override: Option<StateOverride>,
    ) -> RpcResult<U256> {
        todo!()
    }

    async fn gas_price(&self) -> RpcResult<U256> {
        if let Some(provider) = self.inner.provider.as_ref() {
            let gas_price = provider.get_gas_price().await.map_err(alloy_error)?;
            return Ok(U256::from(gas_price));
        }
        Ok(parse_units("1", "gwei").unwrap().get_absolute())
    }

    async fn get_account(&self, address: Address, block: BlockId) -> RpcResult<Option<Account>> {
        let key = format!("{}{}{}", "get_account", address, block);
        // cache
        if let Some(ret) = self.inner.recorder.get(&key).await.map_err(eyre_error)? {
            return Ok(Some(ret));
        }
        // real provider
        if let Some(provider) = self.inner.provider.as_ref() {
            let result = provider.get_account(address).block_id(block).await.map_err(alloy_error)?;
            self.inner.recorder.record(&key, &result).await.map_err(eyre_error)?;
            return Ok(Some(result));
        }
        Ok(None)
    }

    async fn max_priority_fee_per_gas(&self) -> RpcResult<U256> {
        todo!()
    }

    async fn blob_base_fee(&self) -> RpcResult<U256> {
        todo!()
    }

    async fn fee_history(
        &self,
        _block_count: U64,
        _newest_block: BlockNumberOrTag,
        _reward_percentiles: Option<Vec<f64>>,
    ) -> RpcResult<FeeHistory> {
        todo!()
    }

    async fn is_mining(&self) -> RpcResult<bool> {
        todo!()
    }

    async fn hashrate(&self) -> RpcResult<U256> {
        todo!()
    }

    async fn get_work(&self) -> RpcResult<Work> {
        todo!()
    }

    async fn submit_hashrate(&self, _hashrate: U256, _id: B256) -> RpcResult<bool> {
        todo!()
    }

    async fn submit_work(&self, _nonce: B64, _pow_hash: B256, _mix_digest: B256) -> RpcResult<bool> {
        todo!()
    }

    async fn send_transaction(&self, _request: TransactionRequest) -> RpcResult<B256> {
        todo!()
    }

    async fn send_raw_transaction(&self, _bytes: Bytes) -> RpcResult<B256> {
        todo!()
    }

    async fn sign(&self, _address: Address, _message: Bytes) -> RpcResult<Bytes> {
        todo!()
    }

    async fn sign_transaction(&self, _transaction: TransactionRequest) -> RpcResult<Bytes> {
        todo!()
    }

    async fn sign_typed_data(&self, _address: Address, _data: TypedData) -> RpcResult<Bytes> {
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
