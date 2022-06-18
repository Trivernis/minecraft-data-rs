pub mod types;


pub use types::{BitField, NativeType, PacketDataType, PacketDataTypes};

#[derive(Deserialize)]
pub struct Protocol {
    pub types: PacketDataTypes,
}
