use crate::api::protocol::Protocol;
use crate::api::tests::get_test_versions;
use crate::models::protocol::{NativeType, PacketDataType};
use std::sync::Arc;
use crate::DataResult;

#[test]
pub fn simple_test() {
    let versions = get_test_versions();
    for x in versions {
        let arc = Arc::new(x);
        let protocol = Protocol::new(arc.clone());
        let protocol1 = protocol.get_protocol();
        match protocol1 {
            Ok(_) => {}
            Err(error) => {
                println!("{:?} On Version {}", error, arc.minecraft_version);
            }
        }
    }
}