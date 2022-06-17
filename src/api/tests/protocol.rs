use std::sync::Arc;
use crate::Api;
use crate::api::protocol::Protocol;
use crate::api::tests::get_test_versions;
use crate::models::protocol::PacketDataType;

#[test]
pub fn simple_test(){
    let versions = get_test_versions();
    for x in versions {
        let protocol = Protocol::new(Arc::new(x));
        let protocol1 = protocol.get_protocol().unwrap();
        for protocol in protocol1.types.types {
            match protocol {
                PacketDataType::Other(other, data) => {
                    println!("{:?} data {:?}", other,data);
                }
                _ => {}
            }
        }
    }
}