
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MyelinError(pub String);
impl fmt::Display for MyelinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}
impl Error for MyelinError {}

pub struct MemoryStore;
impl MemoryStore {
    pub async fn connect(_url: &str, _path: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self)
    }
    pub async fn sync_delta(&self) -> Result<Vec<omega_core::MemoryNode>, Box<dyn Error + Send + Sync>> {
        Ok(Vec::new())
    }
}
