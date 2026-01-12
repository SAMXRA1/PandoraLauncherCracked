use std::sync::Arc;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ModsToml {
    pub mods: Vec<ModsTomlMod>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModsTomlMod {
    pub mod_id: Arc<str>,
    pub display_name: Option<Arc<str>>,
    pub logo_file: Option<Arc<str>>,
    pub version: Option<Arc<str>>,
    pub authors: Option<Arc<str>>,
}

#[derive(Deserialize, Debug)]
pub struct JarJarMetadata {
    pub jars: Vec<JarJarMetadataJar>
}

#[derive(Deserialize, Debug)]
pub struct JarJarMetadataJar {
    pub path: Arc<str>,
}
