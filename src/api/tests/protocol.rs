use std::collections::HashMap;
use std::convert::TryInto;
use crate::api::protocol::Protocol;
use crate::api::tests::get_test_versions;
use crate::models::protocol::{PacketDataType};
use std::sync::Arc;

pub const VERSIONS_TO_SKIP: [&str; 3] = ["21w07a", "20w14a", "20w13b"];

#[test]
pub fn simple_test() {
    let versions = get_test_versions();
    for x in versions {
        if VERSIONS_TO_SKIP.contains(&x.minecraft_version.as_str()) {
            continue;
        }
        let arc = Arc::new(x);
        let protocol = Protocol::new(arc.clone());
        let protocol1 = protocol.get_protocol();
        match protocol1 {
            Ok(data) => {
                for x in data.play.to_server.types {
                    println!("{:#?}", x);
                }
            }
            Err(error) => {
                panic!("Minecraft Version {} could not be parsed into a Protocol object: {}", arc.minecraft_version, error);
            }
        }
    }
}