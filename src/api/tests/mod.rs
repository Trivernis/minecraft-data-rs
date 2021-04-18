use crate::api::versions::{available_versions, versions};
use crate::api::Api;
use crate::models::version::Version;

mod enchantments;
mod items;
mod loot;
mod recipes;
mod versions;
mod blocks;

fn get_api(version: Version) -> Api {
    Api::new(version)
}

fn get_test_versions() -> Vec<Version> {
    let available = available_versions().unwrap();
    versions()
        .unwrap()
        .into_iter()
        .filter(|v| available.contains(&v.minecraft_version))
        .filter(|v| v.version >= 477) // datapaths < 1.14 are incomplete
        .collect()
}
