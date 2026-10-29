use crate::{
    core::api_client::plugins::McPluginReport,
    setup::{ConfigManager, constants::USER_AGENT},
};
use dashmap::DashMap;
use reqwest::Client;
use serde::{
    Serialize,
    de::{DeserializeOwned, Error},
};
use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ApiProvider {
    #[default]
    Official,
    BMCLApi,
}

#[derive(Clone)]
pub struct ApiBases {
    pub meta_base: &'static str,
    pub resources_base: &'static str,
    pub forge_base: &'static str,
    pub fabric_base: &'static str,
    pub modrinth_base: &'static str,
}

impl ApiBases {
    pub fn new(provider: &ApiProvider) -> Self {
        match provider {
            ApiProvider::Official => ApiBases {
                meta_base: "https://piston-meta.mojang.com",
                resources_base: "https://resources.download.minecraft.net",
                forge_base: "https://maven.minecraftforge.net/net/minecraftforge/forge",
                fabric_base: "https://meta.fabricmc.net/v2/versions/loader",
                modrinth_base: "https://api.modrinth.com/v2",
            },
            ApiProvider::BMCLApi => ApiBases {
                meta_base: "https://bmclapi2.bangbang93.com",
                resources_base: "https://bmclapi2.bangbang93.com/assets",
                forge_base: "https://bmclapi2.bangbang93.com/forge",
                fabric_base: "https://bmclapi2.bangbang93.com/fabric-meta/v2/versions/loader",
                modrinth_base: "https://api.modrinth.com/v2",
            },
        }
    }
    /// Mirror only Mojang-owned download hosts; third-party URLs retain their origin.
    pub fn download_url(&self, url: &str) -> String {
        if self.meta_base == "https://piston-meta.mojang.com" {
            return url.into();
        }
        let Ok(parsed) = reqwest::Url::parse(url) else {
            return url.into();
        };
        let prefix = match parsed.host_str() {
            Some(
                "piston-meta.mojang.com"
                | "launchermeta.mojang.com"
                | "piston-data.mojang.com"
                | "launcher.mojang.com",
            ) => self.meta_base.to_owned(),
            Some("libraries.minecraft.net") => format!("{}/maven", self.meta_base),
            Some("resources.download.minecraft.net") => self.resources_base.to_owned(),
            _ => return url.into(),
        };
        let mut rewritten = format!("{prefix}{}", parsed.path());
        if let Some(query) = parsed.query() {
            rewritten.push('?');
            rewritten.push_str(query);
        }
        rewritten
    }
}

#[derive(thiserror::Error, Debug)]
pub enum McApiError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid Minecraft metadata: {0}")]
    InvalidMetadata(String),

    #[error("Version not found: {0}")]
    VersionNotFound(String),

    #[error("sha1 check failed: {0}")]
    Sha1Mismatch(PathBuf),

    #[error("xml parse error: {0}")]
    XmlError(#[from] quick_xml::DeError),

    #[error("plugin type {0:?} not supported")]
    PluginMismatch(super::mcmod::PluginType),
}

/// Result type for Minecraft API operations.
pub type McApiResult<T> = Result<T, McApiError>;

pub mod game {
    pub use crate::core::minecraft::*;
}

pub mod plugins {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct McPluginReport {
        pub version: String,
        pub stable: Option<bool>,
    }

    pub mod forge_xml {
        use crate::core::api_client::plugins::McPluginReport;

        #[derive(serde::Deserialize)]
        pub struct Metadata {
            versioning: Versioning,
        }

        #[derive(serde::Deserialize)]
        struct Versioning {
            versions: Versions,
        }

        #[derive(Debug, serde::Deserialize)]
        struct Versions {
            version: Vec<String>,
        }

        impl Metadata {
            pub fn find_versions_of_game(&self, mc_version: &str) -> Vec<McPluginReport> {
                let mut versions = Vec::new();
                for version in &self.versioning.versions.version {
                    let version_split = version.split_once('-');
                    if let Some((mc_version_i, forge_version)) = version_split
                        && mc_version_i == mc_version
                    {
                        versions.push(McPluginReport {
                            version: forge_version.to_string(),
                            stable: None,
                        });
                    }
                }
                versions
            }
        }
    }

    pub mod forge_bmcl {
        #[derive(serde::Deserialize, serde::Serialize)]
        pub struct ForgeVersion {
            mcversion: String,
            modified: String,
            pub version: String,
            files: Vec<File>,
        }

        #[derive(serde::Deserialize, serde::Serialize)]
        struct File {
            format: String,
            category: String,
            hash: String,
        }
    }
}

pub struct MinecraftApiClient {
    client: Client,
    api_bases: RwLock<ApiBases>,
    cache: DashMap<String, (Instant, serde_json::Value)>,
    ttl: Duration,
}

impl MinecraftApiClient {
    /// Create a new MinecraftApiClient
    pub fn new(client: Client, api_provider: &ApiProvider) -> Self {
        Self {
            client,
            api_bases: RwLock::new(ApiBases::new(api_provider)),
            cache: DashMap::new(),
            ttl: Duration::from_secs(60 * 5),
        }
    }

    #[cfg(test)]
    pub(super) fn with_test_bases(bases: ApiBases) -> Self {
        let mut client = Self::new(Client::new(), &ApiProvider::Official);
        client.api_bases = RwLock::new(bases);
        client
    }

    /// the provider should be managed in setup info, this will be called once the setup changed
    pub fn switch_provider(&self, provider: &ApiProvider) {
        let mut guard = self.api_bases.blocking_write();
        *guard = ApiBases::new(provider);
        drop(guard);
    }

    pub async fn api_bases_async(&self) -> ApiBases {
        let guard = self.api_bases.read().await;
        guard.clone()
    }

    /// Get data from a URL
    async fn get_inner<T: DeserializeOwned>(&self, url: &str) -> McApiResult<T> {
        let actual_url = self.api_bases.read().await.download_url(url);
        let response = self
            .client
            .get(&actual_url)
            .timeout(Duration::from_secs(60))
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .send()
            .await?
            .error_for_status()?;
        let data: T = response.json().await?;
        log::info!("got data from {:?}", url);
        Ok(data)
    }

    /// Get data from a URL with cache
    pub async fn get<T: DeserializeOwned + Serialize>(
        &self,
        url: &str,
        allow_from_cache: bool,
    ) -> McApiResult<T> {
        let now = Instant::now();
        if allow_from_cache {
            if let Some(entry) = self.cache.get(url) {
                let (cached_at, value) = entry.value();
                if now.duration_since(*cached_at) < self.ttl {
                    log::info!("cache hit for {}", url);
                    return Ok(serde_json::from_value(value.clone())?);
                }
            }
        }
        let data = self.get_inner(url).await?;
        let value = serde_json::to_value(&data)?;
        self.cache.insert(url.to_string(), (now, value));
        Ok(data)
    }

    /// Get data from a endpoint with embedded base_url
    pub async fn get_with_endpoint<T: DeserializeOwned + Serialize>(
        &self,
        endpoint: &str,
        allow_from_cache: bool,
    ) -> McApiResult<T> {
        let url = format!("{}/{}", self.api_bases.read().await.meta_base, endpoint);
        self.get(&url, allow_from_cache).await
    }

    /// Get version details from version id and save the temp
    pub async fn get_version_details(
        &self,
        version_id: &str,
        temp_dir: &Path,
    ) -> McApiResult<game::VersionDetails> {
        crate::util::file::validate_file_name(version_id)
            .map_err(|error| McApiError::InvalidMetadata(error.to_string()))?;
        let version_json = self.get_version_metadata(version_id).await?;
        let details = serde_json::from_value(version_json.clone())?;
        tokio::fs::create_dir_all(temp_dir).await?;
        tokio::fs::write(
            temp_dir.join(format!("{version_id}.json")),
            serde_json::to_vec_pretty(&version_json)?,
        )
        .await?;
        Ok(details)
    }

    /// Preserve every upstream field, including launch arguments and Java requirements.
    pub async fn get_version_metadata(&self, version_id: &str) -> McApiResult<serde_json::Value> {
        use sha1::Digest;
        let manifest: game::VersionManifest = self
            .get_with_endpoint(game::VERSION_MANIFEST_ENDPOINT, true)
            .await?;
        let version = manifest
            .versions
            .iter()
            .find(|v| v.id == version_id)
            .ok_or_else(|| McApiError::VersionNotFound(version_id.into()))?;
        let bytes = self.get_bytes(&version.url).await?;
        if let Some(expected) = &version.sha1 {
            let actual = format!("{:x}", sha1::Sha1::digest(&bytes));
            if !actual.eq_ignore_ascii_case(expected) {
                return Err(McApiError::InvalidMetadata(
                    "Version JSON SHA-1 mismatch".into(),
                ));
            }
        }
        let value: serde_json::Value = serde_json::from_slice(&bytes)?;
        if value["id"].as_str() != Some(version_id) {
            return Err(McApiError::InvalidMetadata(
                "Version metadata ID mismatch".into(),
            ));
        }
        Ok(value)
    }

    /// Get forge version details
    pub async fn get_forge_versions(&self, version_id: &str) -> McApiResult<Vec<McPluginReport>> {
        let guard = ConfigManager::instance().app_state.lock().await;
        let current_provider = &guard.pcl_setup_info.api_provider;
        let forge_base = self.api_bases.read().await.forge_base;
        match *current_provider {
            ApiProvider::Official => {
                drop(guard);
                let url = format!("{forge_base}/maven-metadata.xml");
                let response = self
                    .client
                    .get(url)
                    .header(reqwest::header::USER_AGENT, USER_AGENT)
                    .send()
                    .await?
                    .error_for_status()?;
                let xml_text = response.text().await?;
                let metadata: plugins::forge_xml::Metadata = quick_xml::de::from_str(&xml_text)?;
                let matched_versions = metadata.find_versions_of_game(version_id);
                return Ok(matched_versions);
            }
            ApiProvider::BMCLApi => {
                drop(guard);
                let url_index = format!("{forge_base}/minecraft");
                let indexes: Vec<String> = self.get(&url_index, true).await?;
                log::debug!("BMCL indexes count: {:?}", indexes.len());
                if indexes.iter().all(|index| index != version_id) {
                    return Err(McApiError::VersionNotFound(version_id.to_string()));
                }
                let forge_info: Vec<plugins::forge_bmcl::ForgeVersion> = self
                    .get(&format!("{forge_base}/minecraft/{version_id}"), true)
                    .await?;
                log::debug!("BMCL forge info count: {:?}", forge_info.len());
                return Ok(forge_info
                    .iter()
                    .map(|info| McPluginReport {
                        version: info.version.clone(),
                        stable: None,
                    })
                    .collect());
            }
        }
    }

    pub async fn get_fabric_versions(&self, version_id: &str) -> McApiResult<Vec<McPluginReport>> {
        let fabric_base = self.api_bases.read().await.fabric_base;
        let meta_url = format!("{fabric_base}/{version_id}");
        let fabric_versions: serde_json::Value = self.get(&meta_url, true).await?;
        let loaders = fabric_versions
            .as_array()
            .ok_or_else(|| serde_json::Error::custom("fabric list failed to parse"))?
            .iter()
            .filter_map(|v| {
                let v = v.as_object()?;
                let loader = v.get("loader")?.as_object()?;
                let version = loader.get("version")?.as_str()?;
                let stable = loader.get("stable")?.as_bool();
                Some(McPluginReport {
                    version: version.to_string(),
                    stable,
                })
            })
            .collect();
        Ok(loaders)
    }

    pub async fn fetch_with_modrinth<T: DeserializeOwned + Serialize>(
        &self,
        endpoint: &str,
    ) -> McApiResult<T> {
        let modrinth_base = self.api_bases.read().await.modrinth_base;
        let url = format!("{modrinth_base}/{endpoint}");
        self.get(&url, true).await
    }

    pub async fn get_bytes(&self, url: &str) -> McApiResult<Vec<u8>> {
        let actual_url = self.api_bases.read().await.download_url(url);
        let response = self
            .client
            .get(&actual_url)
            .timeout(Duration::from_secs(60))
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .send()
            .await?
            .error_for_status()?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}

#[tauri::command]
pub async fn fetch_with_modrinth(
    api_client: tauri::State<'_, &MinecraftApiClient>,
    endpoint: &str,
) -> Result<serde_json::Value, String> {
    api_client
        .fetch_with_modrinth(endpoint)
        .await
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirrors_known_official_hosts_without_rewriting_third_party_origins() {
        let bases = ApiBases::new(&ApiProvider::BMCLApi);
        for (url, expected) in [
            (
                "https://piston-meta.mojang.com/v1/test.json",
                "https://bmclapi2.bangbang93.com/v1/test.json",
            ),
            (
                "https://piston-data.mojang.com/v1/client.jar",
                "https://bmclapi2.bangbang93.com/v1/client.jar",
            ),
            (
                "https://libraries.minecraft.net/org/test.jar",
                "https://bmclapi2.bangbang93.com/maven/org/test.jar",
            ),
            (
                "https://resources.download.minecraft.net/ab/hash",
                "https://bmclapi2.bangbang93.com/assets/ab/hash",
            ),
            (
                "https://example.org/test.jar?file=1",
                "https://example.org/test.jar?file=1",
            ),
            (
                "https://libraries.minecraft.net.evil.test/a",
                "https://libraries.minecraft.net.evil.test/a",
            ),
        ] {
            assert_eq!(bases.download_url(url), expected);
        }
    }

    #[tokio::test]
    async fn mc_manifest() {
        let mc_api_client = MinecraftApiClient::new(Client::new(), &ApiProvider::Official);
        let manifest: game::VersionManifest = mc_api_client
            .get_with_endpoint(game::VERSION_MANIFEST_ENDPOINT, true)
            .await
            .unwrap();
        println!("{:?}", manifest);
    }

    #[test]
    fn forge_test() {
        let mc_api_client = &ConfigManager::instance().api_client;
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let re = mc_api_client.get_forge_versions("1.21.9").await;
            assert!(re.is_ok());
            println!("{re:?}");
        });
    }
}
