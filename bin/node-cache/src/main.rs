use crate::arguments::AppArgs;
use alloy_provider::ProviderBuilder;
use clap::Parser;
use jsonrpsee::server::ServerBuilder;
use node_cache_recorder::{PickleRecorder, Recorder};
use node_cache_rpc::{NodeCacheDebugApi, NodeCacheEthApi};
use reth_rpc_api::{DebugApiServer, EthApiServer};
use std::process::exit;
use std::sync::Arc;
use tokio::signal;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;

mod arguments;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    tracing_subscriber::FmtSubscriber::builder().with_env_filter(env_filter).finish().try_init()?;
    let args = AppArgs::parse();

    let mut provider = None;
    // Real node
    if let Some(node_http) = args.node {
        provider = Some(ProviderBuilder::new().on_http(node_http.parse()?).boxed());
    }

    info!("DB file path: {:?}", std::path::absolute(&args.db_file_path)?);
    let recorder = Arc::new(PickleRecorder::new(args.db_file_path));

    // APIs
    let debug_eth = NodeCacheDebugApi::new(provider.clone(), recorder.clone());
    let core_eth = NodeCacheEthApi::new(provider.clone(), recorder.clone());
    let mut rpc_module = core_eth.into_rpc();
    rpc_module.merge(debug_eth.into_rpc())?;

    // Server
    let server = ServerBuilder::default().build(args.host).await?;
    let addr = server.local_addr()?;
    let handle = server.start(rpc_module);

    info!("Server started at {:?}", addr);

    let recorder_clone = recorder.clone();
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("failed to listen for event");
        info!("Received ctrl-c, shutting down server");
        handle.stop().unwrap();
        recorder_clone.save().await.expect("failed to save recorder");
        info!("Recorder saved, exiting");
        exit(0)
    });

    futures::future::pending::<()>().await;
    Ok(())
}
