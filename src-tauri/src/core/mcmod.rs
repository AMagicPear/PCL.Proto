#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginType {
    Vanilla,
    Forge,
    NeoForge,
    Fabric,
    LegacyFabric,
    Quilt,
    Optifine,
}

impl Default for PluginType {
    fn default() -> Self {
        PluginType::Vanilla
    }
}
