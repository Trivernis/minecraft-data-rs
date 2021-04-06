use crate::models::version::Version;
use crate::{DataError, DataResult};
use include_dir::Dir;

pub static MINECRAFT_DATA: Dir = include_dir::include_dir!("minecraft-data/data/pc");

pub static BIOMES_FILE: &str = "biomes.json";
pub static BLOCK_LOOT_FILE: &str = "blockLoot.json";
pub static BLOCKS_FILE: &str = "blocks.json";
pub static COMMANDS_FILE: &str = "commands.json";
pub static ENTITIES_FILE: &str = "entities.json";
pub static ENTITY_LOOT_FILE: &str = "entityLoot.json";
pub static ITEMS_FILE: &str = "items.json";
pub static LOGIN_PACKET_FILE: &str = "loginPacket.json";
pub static MATERIALS_FILE: &str = "materials.json";
pub static PROTOCOL_FILE: &str = "protocol.json";
pub static RECIPES_FILE: &str = "recipes.json";
pub static TINTS_FILE: &str = "tints.json";
// pub static VERSION_FILE: &str = "version.json";
pub static MAP_ICONS_FILE: &str = "mapIcons.json";
pub static PARTICLES_FILE: &str = "particles.json";
pub static PROTOCOL_VERSIONS_FILE: &str = "protocolVersions.json";
pub static VERSIONS_FILE: &str = "versions.json";

/// Returns the string encoded content of the common file
pub fn get_common_file(filename: &str) -> DataResult<String> {
    MINECRAFT_DATA
        .get_file(format!("common/{}", filename))
        .ok_or(DataError::NotFoundError(filename.to_string()))?
        .contents_utf8()
        .ok_or(DataError::InvalidEncodingError(filename.to_string()))
        .map(|d| d.to_string())
}

/// Returns the string encoded content of the version specific file
pub fn get_version_specific_file(version: &Version, filename: &str) -> DataResult<String> {
    let search_folders = vec![
        version.minecraft_version.clone(),
        version.major_version.clone(),
        version.major_first(),
        version.previous_major(),
        version.previous_major_first(),
    ];
    println!("{:?}", search_folders);
    let mut data = None;

    for folder in search_folders {
        data = MINECRAFT_DATA.get_file(format!("{}/{}", folder, filename));
        if data.is_some() {
            break;
        }
    }

    data.ok_or(DataError::NotFoundError(format!(
        "{}/{}",
        version.minecraft_version, filename
    )))?
    .contents_utf8()
    .ok_or(DataError::InvalidEncodingError(filename.to_string()))
    .map(|d| d.to_string())
}
