use crate::helper::base::{build_inner, ApiInner};
use crate::helper::error::{alloy_error, eyre_error};
use alloy_primitives::{Address, Bytes, B256};
use alloy_provider::ext::DebugApi;
use alloy_provider::network::Ethereum;
use alloy_provider::Provider;
use alloy_rpc_types_debug::ExecutionWitness;
use alloy_rpc_types_eth::{Block, BlockId, BlockNumberOrTag, Bundle, StateContext, TransactionRequest};
use alloy_rpc_types_trace::geth::{
    BlockTraceResult, DefaultFrame, GethDebugTracingCallOptions, GethDebugTracingOptions, GethTrace, TraceResult,
};
use alloy_transport::Transport;
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use node_cache_recorder::Recorder;
use reth_rpc_api::DebugApiServer;
use std::sync::Arc;

pub struct NodeCacheDebugApi<T, P, R>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Send + Sync + Clone + 'static,
    R: Recorder,
{
    inner: ApiInner<T, P, R>,
}

impl<T, P, R> NodeCacheDebugApi<T, P, R>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Send + Sync + Clone + 'static,
    R: Recorder,
{
    pub fn new(provider: Option<P>, recorder: Arc<R>) -> Self {
        Self { inner: build_inner(provider, recorder) }
    }
}

#[async_trait]
impl<T, P, R> DebugApiServer for NodeCacheDebugApi<T, P, R>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Send + Sync + Clone + 'static,
    R: Recorder + Sync + Send + 'static,
{
    async fn raw_header(&self, _block_id: BlockId) -> RpcResult<Bytes> {
        todo!()
    }

    async fn raw_block(&self, _block_id: BlockId) -> RpcResult<Bytes> {
        todo!()
    }

    async fn raw_transaction(&self, _hash: B256) -> RpcResult<Option<Bytes>> {
        todo!()
    }

    async fn raw_transactions(&self, _block_id: BlockId) -> RpcResult<Vec<Bytes>> {
        todo!()
    }

    async fn raw_receipts(&self, _block_id: BlockId) -> RpcResult<Vec<Bytes>> {
        todo!()
    }

    async fn bad_blocks(&self) -> RpcResult<Vec<Block>> {
        todo!()
    }

    async fn debug_trace_chain(
        &self,
        _start_exclusive: BlockNumberOrTag,
        _end_inclusive: BlockNumberOrTag,
    ) -> RpcResult<Vec<BlockTraceResult>> {
        todo!()
    }

    async fn debug_trace_block(&self, _rlp_block: Bytes, _opts: Option<GethDebugTracingOptions>) -> RpcResult<Vec<TraceResult>> {
        todo!()
    }

    async fn debug_trace_block_by_hash(&self, _block: B256, _opts: Option<GethDebugTracingOptions>) -> RpcResult<Vec<TraceResult>> {
        todo!()
    }

    async fn debug_trace_block_by_number(
        &self,
        block: BlockNumberOrTag,
        opts: Option<GethDebugTracingOptions>,
    ) -> RpcResult<Vec<TraceResult>> {
        let key = format!("{}{}{:?}", "debug_trace_block_by_number", block, opts);
        // cache
        if let Some(ret) = self.inner.recorder.get(&key).await.map_err(eyre_error)? {
            return Ok(ret);
        }
        // real provider
        if let Some(provider) = self.inner.provider.as_ref() {
            let result = match opts {
                Some(opts) => provider.debug_trace_block_by_number(block, opts).await.map_err(alloy_error)?,
                None => {
                    let opts = GethDebugTracingOptions::default();
                    provider.debug_trace_block_by_number(block, opts).await.map_err(alloy_error)?
                }
            };

            self.inner.recorder.record(&key, &result).await.map_err(eyre_error)?;
            return Ok(result);
        }
        Ok(vec![])
    }

    async fn debug_trace_transaction(&self, _tx_hash: B256, _opts: Option<GethDebugTracingOptions>) -> RpcResult<GethTrace> {
        todo!()
    }

    async fn debug_trace_call(
        &self,
        request: TransactionRequest,
        block_id: Option<BlockId>,
        opts: Option<GethDebugTracingCallOptions>,
    ) -> RpcResult<GethTrace> {
        let key = format!("{}{:?}{:?}{:?}", "debug_trace_call", request, block_id, opts);
        // cache
        if let Some(ret) = self.inner.recorder.get(&key).await.map_err(eyre_error)? {
            return Ok(ret);
        }
        // real provider
        if let Some(provider) = self.inner.provider.as_ref() {
            let result = match opts {
                Some(opts) => provider.debug_trace_call(request, block_id.unwrap(), opts).await.map_err(alloy_error)?,
                None => {
                    let opts = GethDebugTracingCallOptions::default();
                    provider.debug_trace_call(request, block_id.unwrap(), opts).await.map_err(alloy_error)?
                }
            };
            self.inner.recorder.record(&key, &result).await.map_err(eyre_error)?;
            return Ok(result);
        }
        Ok(GethTrace::Default(DefaultFrame::default()))
    }

    async fn debug_trace_call_many(
        &self,
        _bundles: Vec<Bundle>,
        _state_context: Option<StateContext>,
        _opts: Option<GethDebugTracingCallOptions>,
    ) -> RpcResult<Vec<Vec<GethTrace>>> {
        todo!()
    }

    async fn debug_execution_witness(&self, _block: BlockNumberOrTag) -> RpcResult<ExecutionWitness> {
        todo!()
    }

    async fn debug_backtrace_at(&self, _location: &str) -> RpcResult<()> {
        todo!()
    }

    async fn debug_account_range(
        &self,
        _block_number: BlockNumberOrTag,
        _start: Bytes,
        _max_results: u64,
        _nocode: bool,
        _nostorage: bool,
        _incompletes: bool,
    ) -> RpcResult<()> {
        todo!()
    }

    async fn debug_block_profile(&self, _file: String, _seconds: u64) -> RpcResult<()> {
        todo!()
    }

    async fn debug_chaindb_compact(&self) -> RpcResult<()> {
        todo!()
    }

    async fn debug_chaindb_property(&self, _property: String) -> RpcResult<()> {
        todo!()
    }

    async fn debug_cpu_profile(&self, _file: String, _seconds: u64) -> RpcResult<()> {
        todo!()
    }

    async fn debug_db_ancient(&self, _kind: String, _number: u64) -> RpcResult<()> {
        todo!()
    }

    async fn debug_db_ancients(&self) -> RpcResult<()> {
        todo!()
    }

    async fn debug_db_get(&self, _key: String) -> RpcResult<()> {
        todo!()
    }

    async fn debug_dump_block(&self, _number: BlockId) -> RpcResult<()> {
        todo!()
    }

    async fn debug_free_os_memory(&self) -> RpcResult<()> {
        todo!()
    }

    async fn debug_freeze_client(&self, _node: String) -> RpcResult<()> {
        todo!()
    }

    async fn debug_gc_stats(&self) -> RpcResult<()> {
        todo!()
    }

    async fn debug_get_accessible_state(&self, _from: BlockNumberOrTag, _to: BlockNumberOrTag) -> RpcResult<()> {
        todo!()
    }

    async fn debug_get_modified_accounts_by_hash(&self, _start_hash: B256, _end_hash: B256) -> RpcResult<()> {
        todo!()
    }

    async fn debug_get_modified_accounts_by_number(&self, _start_number: u64, _end_number: u64) -> RpcResult<()> {
        todo!()
    }

    async fn debug_go_trace(&self, _file: String, _seconds: u64) -> RpcResult<()> {
        todo!()
    }

    async fn debug_intermediate_roots(&self, _block_hash: B256, _opts: Option<GethDebugTracingCallOptions>) -> RpcResult<()> {
        todo!()
    }

    async fn debug_mem_stats(&self) -> RpcResult<()> {
        todo!()
    }

    async fn debug_mutex_profile(&self, _file: String, _nsec: u64) -> RpcResult<()> {
        todo!()
    }

    async fn debug_preimage(&self, _hash: B256) -> RpcResult<()> {
        todo!()
    }

    async fn debug_print_block(&self, _number: u64) -> RpcResult<()> {
        todo!()
    }

    async fn debug_seed_hash(&self, _number: u64) -> RpcResult<B256> {
        todo!()
    }

    async fn debug_set_block_profile_rate(&self, _rate: u64) -> RpcResult<()> {
        todo!()
    }

    async fn debug_set_gc_percent(&self, _v: i32) -> RpcResult<()> {
        todo!()
    }

    async fn debug_set_head(&self, _number: u64) -> RpcResult<()> {
        todo!()
    }

    async fn debug_set_mutex_profile_fraction(&self, _rate: i32) -> RpcResult<()> {
        todo!()
    }

    async fn debug_set_trie_flush_interval(&self, _interval: String) -> RpcResult<()> {
        todo!()
    }

    async fn debug_stacks(&self) -> RpcResult<()> {
        todo!()
    }

    async fn debug_standard_trace_bad_block_to_file(
        &self,
        _block: BlockNumberOrTag,
        _opts: Option<GethDebugTracingCallOptions>,
    ) -> RpcResult<()> {
        todo!()
    }

    async fn debug_standard_trace_block_to_file(
        &self,
        _block: BlockNumberOrTag,
        _opts: Option<GethDebugTracingCallOptions>,
    ) -> RpcResult<()> {
        todo!()
    }

    async fn debug_start_cpu_profile(&self, _file: String) -> RpcResult<()> {
        todo!()
    }

    async fn debug_start_go_trace(&self, _file: String) -> RpcResult<()> {
        todo!()
    }

    async fn debug_stop_cpu_profile(&self) -> RpcResult<()> {
        todo!()
    }

    async fn debug_stop_go_trace(&self) -> RpcResult<()> {
        todo!()
    }

    async fn debug_storage_range_at(
        &self,
        _block_hash: B256,
        _tx_idx: usize,
        _contract_address: Address,
        _key_start: B256,
        _max_result: u64,
    ) -> RpcResult<()> {
        todo!()
    }

    async fn debug_trace_bad_block(&self, _block_hash: B256, _opts: Option<GethDebugTracingCallOptions>) -> RpcResult<()> {
        todo!()
    }

    async fn debug_verbosity(&self, _level: usize) -> RpcResult<()> {
        todo!()
    }

    async fn debug_vmodule(&self, _pattern: String) -> RpcResult<()> {
        todo!()
    }

    async fn debug_write_block_profile(&self, _file: String) -> RpcResult<()> {
        todo!()
    }

    async fn debug_write_mem_profile(&self, _file: String) -> RpcResult<()> {
        todo!()
    }

    async fn debug_write_mutex_profile(&self, _file: String) -> RpcResult<()> {
        todo!()
    }
}
