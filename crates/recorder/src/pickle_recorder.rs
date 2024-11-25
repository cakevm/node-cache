use crate::Recorder;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::RwLock;

pub struct PickleRecorder {
    db: RwLock<PickleDb>,
}

impl PickleRecorder {
    pub fn new(db_path: String) -> Self {
        let db = PickleDb::new(db_path, PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json);
        PickleRecorder { db: RwLock::new(db) }
    }
}
//tokio::task::spawn_blocking(move || {
#[async_trait::async_trait]
impl Recorder for PickleRecorder {
    async fn record<T: Serialize + Send + Sync>(&self, key: &str, value: &T) -> eyre::Result<()> {
        Ok(self.db.write().await.set(key, &value)?)
    }

    async fn get<T: DeserializeOwned>(&self, key: &str) -> eyre::Result<Option<T>> {
        Ok(self.db.read().await.get::<T>(key))
    }

    async fn save(&self) -> eyre::Result<()> {
        Ok(self.db.write().await.dump()?)
    }
}
