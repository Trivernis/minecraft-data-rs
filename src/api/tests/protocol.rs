use std::sync::Arc;

use crate::api::protocol::Protocol;
use crate::api::tests::get_test_versions;

#[test]
pub fn test_get_protocol() {
    let versions = get_test_versions();

    for x in versions {
        let arc = Arc::new(x);
        let protocol = Protocol::new(arc.clone());

        if let Err(e) = protocol.get_protocol() {
            panic!(
                "Minecraft Version {} could not be parsed into a Protocol object: {}",
                arc.minecraft_version, e
            );
        }
    }
}
