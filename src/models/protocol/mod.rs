pub mod types;
pub use types::{NativeType, PacketDataTypes, PacketDataType,BitField};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct Protocol {
    pub types: PacketDataTypes,
}

