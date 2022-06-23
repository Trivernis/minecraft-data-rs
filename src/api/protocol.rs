use crate::data::{get_version_specific_file, PROTOCOL_FILE};
use crate::models::version::Version;
use crate::{DataError, DataResult};
use std::sync::Arc;

pub struct Protocol {
    version: Arc<Version>,
}

impl Protocol {
    pub fn new(version: Arc<Version>) -> Self {
        Self { version }
    }

    /// Returns the protocol information for the current version
    pub fn get_protocol(&self) -> DataResult<crate::models::protocol::Protocol> {
        let content = get_version_specific_file(&self.version, PROTOCOL_FILE)?;
        serde_json::from_str(&content).map_err(DataError::from)
    }
}
