#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModLoaderType {
    Vanilla,
    Forge,
    NeoForge,
    Fabric,
    LegacyFabric,
    Quilt,
    Optifine,
}

impl Default for ModLoaderType {
    fn default() -> Self {
        ModLoaderType::Vanilla
    }
}
