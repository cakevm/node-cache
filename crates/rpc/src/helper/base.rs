use alloy_provider::network::Ethereum;
use alloy_provider::Provider;
use alloy_transport::Transport;
use node_cache_recorder::Recorder;
use std::sync::Arc;

pub fn build_inner<T, P, R>(provider: Option<P>, recorder: Arc<R>) -> ApiInner<T, P, R>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Send + Sync + Clone + 'static,
    R: Recorder,
{
    ApiInner { provider, _t: std::marker::PhantomData, recorder }
}

#[derive(Debug, Clone)]
pub struct ApiInner<T, P, R>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Send + Sync + Clone + 'static,
    R: Recorder,
{
    pub(crate) provider: Option<P>,
    _t: std::marker::PhantomData<T>,
    pub(crate) recorder: Arc<R>,
}

impl<T, P, R> ApiInner<T, P, R>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum> + Send + Sync + Clone + 'static,
    R: Recorder,
{
}
