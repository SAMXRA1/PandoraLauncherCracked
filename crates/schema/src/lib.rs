use serde::Deserialize;

pub mod assets_index;
pub mod content;
pub mod fabric_launch;
pub mod fabric_loader_manifest;
pub mod instance;
pub mod java_runtime_component;
pub mod java_runtimes;
pub mod loader;
pub mod modification;
pub mod modrinth;
pub mod version;
pub mod version_manifest;

pub fn try_deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + Default,
    D: serde::Deserializer<'de>,
{
    Ok(T::deserialize(serde_json::Value::deserialize(deserializer)?).unwrap_or_default())
}
