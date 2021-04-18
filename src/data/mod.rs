mod datapaths;

use crate::data::datapaths::Datapaths;
use crate::models::version::Version;
use crate::{DataError, DataResult};
use include_dir::Dir;

pub static MINECRAFT_DATA: Dir = include_dir::include_dir!("minecraft-data/data");

pub static BIOMES_FILE: &str = "biomes";
pub static BLOCK_LOOT_FILE: &str = "blockLoot";
pub static BLOCKS_FILE: &str = "blocks";
pub static COMMANDS_FILE: &str = "commands";
pub static ENTITIES_FILE: &str = "entities";
pub static ENTITY_LOOT_FILE: &str = "entityLoot";
pub static ITEMS_FILE: &str = "items";
pub static LOGIN_PACKET_FILE: &str = "loginPacket";
pub static MATERIALS_FILE: &str = "materials";
pub static PROTOCOL_FILE: &str = "protocol";
pub static RECIPES_FILE: &str = "recipes";
pub static TINTS_FILE: &str = "tints";
pub static ENCHANTMENTS_FILE: &str = "enchantments";
// pub static VERSION_FILE: &str = "version.json";
pub static MAP_ICONS_FILE: &str = "mapIcons";
pub static PARTICLES_FILE: &str = "particles";
pub static PROTOCOL_VERSIONS_FILE: &str = "protocolVersions";
pub static VERSIONS_FILE: &str = "versions";

/// Returns the string encoded content of the common file
pub fn get_common_file(filename: &str) -> DataResult<String> {
    MINECRAFT_DATA
        .get_file(format!("pc/common/{}.json", filename))
        .ok_or(DataError::NotFoundError(filename.to_string()))?
        .contents_utf8()
        .ok_or(DataError::InvalidEncodingError(filename.to_string()))
        .map(|d| d.to_string())
}

/// Returns the string encoded content of the version specific file
pub fn get_version_specific_file(version: &Version, filename: &str) -> DataResult<String> {
    let path = get_path(version, filename)?;
    MINECRAFT_DATA
        .get_file(format!("{}/{}.json", path, filename))
        .ok_or(DataError::NotFoundError(format!(
            "{}/{}",
            version.minecraft_version, filename
        )))?
        .contents_utf8()
        .ok_or(DataError::InvalidEncodingError(filename.to_string()))
        .map(|d| d.to_string())
}

/// Returns the data path for a given file
pub fn get_path(version: &Version, filename: &str) -> DataResult<String> {
    lazy_static::lazy_static! {
         static ref PATHS: Datapaths = get_datapaths().unwrap();
    };
    PATHS
        .pc
        .get(&version.minecraft_version)
        .ok_or(DataError::NotFoundError(version.minecraft_version.clone()))?
        .get(filename)
        .cloned()
        .ok_or(DataError::NotFoundError(filename.to_string()))
}

/// Returns the parsed data paths
fn get_datapaths() -> DataResult<Datapaths> {
    let content = MINECRAFT_DATA
        .get_file("dataPaths.json")
        .ok_or(DataError::NotFoundError("dataPaths.json".to_string()))?
        .contents_utf8()
        .ok_or(DataError::InvalidEncodingError(
            "dataPaths.json".to_string(),
        ))?;
    serde_json::from_str::<Datapaths>(content).map_err(DataError::from)
}
