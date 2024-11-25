use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait::async_trait]
pub trait Recorder {
    async fn record<T: Serialize + Send + Sync>(&self, key: &str, value: &T) -> eyre::Result<()>;
    async fn get<T: DeserializeOwned>(&self, key: &str) -> eyre::Result<Option<T>>;
    async fn save(&self) -> eyre::Result<()>;
}
