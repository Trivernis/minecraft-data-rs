use crate::data::{get_version_specific_file, PROTOCOL_FILE};
use crate::models::version::Version;
use crate::{DataError, DataResult};
use std::sync::Arc;


/// Will not parse versions `21w07a`, `20w14a`, and `20w13b`
/// These snapshot versions have incompatible data types for the tags packet. 

pub struct Protocol {
    version: Arc<Version>,
}

impl Protocol {
    pub fn new(version: Arc<Version>) -> Self {
        Self { version }
    }
    pub fn get_protocol(&self) -> DataResult<crate::models::protocol::Protocol> {
        let content = get_version_specific_file(&self.version, PROTOCOL_FILE)?;
        serde_json::from_str(&content).map_err(DataError::from)
    }
}
