//! Mojang version metadata and the rules shared by installation and launch.
use std::collections::BTreeMap;

use anyhow::{Context, bail, ensure};
use serde::{Deserialize, Serialize};

pub const VERSION_MANIFEST_ENDPOINT: &str = "mc/game/version_manifest_v2.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionManifest {
    pub latest: LatestVersions,
    pub versions: Vec<VersionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestVersions {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    pub id: String,
    pub r#type: String,
    pub url: String,
    pub time: String,
    pub release_time: String,
    #[serde(default)]
    pub sha1: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionDetails {
    pub id: String,
    pub r#type: String,
    pub downloads: VersionDownloads,
    pub libraries: Vec<LibraryItem>,
    pub assets: String,
    pub asset_index: AssetIndexInfo,
    pub main_class: String,
    pub logging: Option<Logging>,
    #[serde(default)]
    pub arguments: Option<VersionArguments>,
    #[serde(default)]
    pub minecraft_arguments: Option<String>,
    #[serde(default)]
    pub release_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VersionArguments {
    #[serde(default)]
    pub game: Vec<Argument>,
    #[serde(default)]
    pub jvm: Vec<Argument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Argument {
    Plain(String),
    Conditional {
        rules: Vec<Rule>,
        value: ArgumentValue,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArgumentValue {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDownloads {
    pub client: DownloadInfo,
    pub server: Option<DownloadInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadInfo {
    #[serde(default)]
    pub sha1: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub url: String,
    pub path: Option<String>,
}

impl DownloadInfo {
    pub fn validate(&self) -> anyhow::Result<()> {
        ensure!(valid_sha1(&self.sha1), "Invalid SHA-1: {}", self.sha1);
        let url = reqwest::Url::parse(&self.url)?;
        ensure!(url.scheme() == "https", "Download URL must use HTTPS");
        ensure!(
            url.username().is_empty() && url.password().is_none(),
            "Download URL contains credentials"
        );
        Ok(())
    }
}

pub fn valid_sha1(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|c| c.is_ascii_hexdigit())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetIndexInfo {
    pub id: String,
    #[serde(flatten)]
    pub download: DownloadInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logging {
    pub client: LoggingClient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingClient {
    pub argument: String,
    pub file: LoggingFile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingFile {
    pub id: String,
    #[serde(flatten)]
    pub download: DownloadInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub action: String,
    pub os: Option<OsRule>,
    pub features: Option<BTreeMap<String, bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsRule {
    pub name: Option<String>,
    pub arch: Option<String>,
    pub version: Option<String>,
}

pub struct RuleContext {
    pub os: String,
    pub arch: String,
    pub version: String,
    pub features: BTreeMap<String, bool>,
}

impl RuleContext {
    pub fn current() -> Self {
        Self {
            os: match std::env::consts::OS {
                "macos" => "osx",
                os => os,
            }
            .into(),
            arch: std::env::consts::ARCH.into(),
            version: tauri_plugin_os::version().to_string(),
            features: BTreeMap::new(),
        }
    }

    pub fn for_java(architecture: &super::platform::Architecture) -> anyhow::Result<Self> {
        use super::platform::Architecture;
        let arch = match architecture {
            Architecture::X86 => "x86",
            Architecture::X64 => "x86_64",
            Architecture::Arm64 => "aarch64",
            Architecture::FatFile | Architecture::Unknown => {
                bail!("Cannot determine the running Java architecture")
            }
        };
        let mut context = Self::current();
        context.arch = arch.into();
        Ok(context)
    }
}

fn pattern_matches(pattern: &Option<String>, value: &str) -> anyhow::Result<bool> {
    match pattern {
        Some(pattern) => Ok(regex::Regex::new(pattern)
            .with_context(|| format!("Invalid Minecraft rule pattern: {pattern}"))?
            .is_match(value)),
        None => Ok(true),
    }
}

/// Mojang rules start disallowed; the last matching rule wins.
pub fn rules_allow(rules: Option<&[Rule]>, context: &RuleContext) -> anyhow::Result<bool> {
    let Some(rules) = rules else { return Ok(true) };
    let mut allowed = false;
    for rule in rules {
        ensure!(
            matches!(rule.action.as_str(), "allow" | "disallow"),
            "Invalid rule action: {}",
            rule.action
        );
        let os_matches = if let Some(os) = &rule.os {
            os.name.as_ref().is_none_or(|name| name == &context.os)
                && pattern_matches(&os.arch, &context.arch)?
                && pattern_matches(&os.version, &context.version)?
        } else {
            true
        };
        let features_match = rule.features.as_ref().is_none_or(|features| {
            features.iter().all(|(name, required)| {
                context.features.get(name).copied().unwrap_or(false) == *required
            })
        });
        if os_matches && features_match {
            allowed = rule.action == "allow";
        }
    }
    Ok(allowed)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryItem {
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub sha1: Option<String>,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub downloads: Option<LibraryDownloads>,
    pub rules: Option<Vec<Rule>>,
    #[serde(default)]
    pub natives: BTreeMap<String, String>,
    pub extract: Option<ExtractRules>,
    #[serde(default)]
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractRules {
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LibraryDownloads {
    pub artifact: Option<DownloadInfo>,
    #[serde(default)]
    pub classifiers: BTreeMap<String, DownloadInfo>,
}

impl LibraryItem {
    pub fn classpath_path(
        &self,
        repo: &std::path::Path,
        instance: &std::path::Path,
    ) -> anyhow::Result<Option<std::path::PathBuf>> {
        if let Some(artifact) = self
            .downloads
            .as_ref()
            .and_then(|downloads| downloads.artifact.as_ref())
            && artifact.path.is_some()
        {
            return Ok(Some(library_path(repo, artifact)?));
        }
        if self
            .downloads
            .as_ref()
            .is_some_and(|downloads| downloads.artifact.is_none())
            || !self.natives.is_empty() && self.downloads.is_none()
        {
            return Ok(None);
        }
        let (coordinate, extension) = self.name.split_once('@').unwrap_or((&self.name, "jar"));
        let parts: Vec<_> = coordinate.split(':').collect();
        ensure!(
            matches!(parts.len(), 3 | 4),
            "Invalid library coordinate: {}",
            self.name
        );
        for part in parts.iter().chain(std::iter::once(&extension)) {
            crate::util::file::validate_file_name(part)?;
        }
        if self.hint.as_deref() == Some("local") {
            let file = format!("{}-{}.{}", parts[1], parts[2..].join("-"), extension);
            return Ok(Some(instance.join("libraries").join(file)));
        }
        let group = parts[0].split('.').collect::<Vec<_>>();
        for part in &group {
            crate::util::file::validate_file_name(part)?;
        }
        let classifier = parts
            .get(3)
            .map(|value| format!("-{value}"))
            .unwrap_or_default();
        let path = format!(
            "{}/{}/{}/{}-{}{}.{}",
            group.join("/"),
            parts[1],
            parts[2],
            parts[1],
            parts[2],
            classifier,
            extension
        );
        Ok(Some(crate::util::file::safe_join(
            &repo.join("libraries"),
            &path,
        )?))
    }

    pub fn classpath_key(&self) -> anyhow::Result<String> {
        let (coordinate, extension) = self.name.split_once('@').unwrap_or((&self.name, "jar"));
        let parts: Vec<_> = coordinate.split(':').collect();
        ensure!(
            matches!(parts.len(), 3 | 4),
            "Invalid library coordinate: {}",
            self.name
        );
        Ok(format!(
            "{}:{}:{}@{}",
            parts[0],
            parts[1],
            parts.get(3).copied().unwrap_or(""),
            extension
        ))
    }

    pub fn rule_allow(&self, context: &RuleContext) -> anyhow::Result<bool> {
        rules_allow(self.rules.as_deref(), context)
    }

    pub fn native_download(&self, context: &RuleContext) -> anyhow::Result<Option<&DownloadInfo>> {
        let Some(classifier) = self.natives.get(&context.os) else {
            return Ok(None);
        };
        let classifier =
            classifier.replace("${arch}", if context.arch == "x86" { "32" } else { "64" });
        match self
            .downloads
            .as_ref()
            .and_then(|downloads| downloads.classifiers.get(&classifier))
        {
            Some(info) => Ok(Some(info)),
            None => bail!("Missing native classifier {classifier} in {}", self.name),
        }
    }
}

pub fn library_path(
    repo: &std::path::Path,
    info: &DownloadInfo,
) -> anyhow::Result<std::path::PathBuf> {
    crate::util::file::safe_join(
        &repo.join("libraries"),
        info.path
            .as_deref()
            .context("Missing library artifact path")?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn context() -> RuleContext {
        RuleContext {
            os: "osx".into(),
            arch: "aarch64".into(),
            version: "14.0".into(),
            features: BTreeMap::new(),
        }
    }

    #[test]
    fn java_rule_context_uses_probed_architecture() {
        use super::super::platform::Architecture;
        for (arch, expected) in [
            (Architecture::X86, "x86"),
            (Architecture::X64, "x86_64"),
            (Architecture::Arm64, "aarch64"),
        ] {
            assert_eq!(RuleContext::for_java(&arch).unwrap().arch, expected);
        }
        assert!(RuleContext::for_java(&Architecture::Unknown).is_err());
        assert!(RuleContext::for_java(&Architecture::FatFile).is_err());
    }

    #[test]
    fn ordered_rules_handle_unconditional_os_arch_version_and_features() {
        let parse = |value| serde_json::from_value::<Vec<Rule>>(value).unwrap();
        let rules = parse(json!([
            {"action":"allow"},
            {"action":"disallow","os":{"name":"osx"}},
            {"action":"allow","os":{"name":"osx","arch":"aarch64","version":"^14\\."}}
        ]));
        assert!(rules_allow(Some(&rules), &context()).unwrap());
        assert!(!rules_allow(Some(&rules[..2]), &context()).unwrap());
        assert!(!rules_allow(Some(&[]), &context()).unwrap());
        assert!(rules_allow(None, &context()).unwrap());
        let rules = parse(json!([{"action":"allow","features":{"is_demo_user":true}}]));
        assert!(!rules_allow(Some(&rules), &context()).unwrap());
        let rules = parse(json!([{"action":"allow","os":{"arch":"["}}]));
        assert!(rules_allow(Some(&rules), &context()).is_err());
    }

    #[test]
    fn classifier_only_libraries_and_missing_servers_are_valid() {
        let library: LibraryItem = serde_json::from_value(json!({
            "name":"org.lwjgl:lwjgl:2", "natives":{"osx":"natives-osx-${arch}"},
            "downloads":{"classifiers":{"natives-osx-64":{"sha1":"a".repeat(40),"size":1,"url":"https://example.org/native.jar"}}}
        })).unwrap();
        assert!(library.downloads.as_ref().unwrap().artifact.is_none());
        assert!(library.native_download(&context()).unwrap().is_some());
        let downloads: VersionDownloads = serde_json::from_value(json!({"client":{"sha1":"a".repeat(40),"size":1,"url":"https://example.org/client.jar"}})).unwrap();
        assert!(downloads.server.is_none());
    }
}
