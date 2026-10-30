//! Persisted launcher state, mirroring `PCL.Core.App.States` groups.

use crate::{
    app::metadata::constants,
    io::net::http::api_client::ApiProvider,
    minecraft::{
        instance::{McInstance, folder::McFolder},
        java::JavaEntry,
        profile::McProfile,
    },
};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::Arc,
};

/// PCL global setup info, which user can modify
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct LauncherSetup {
    setup_version: i32,
    theme: Theme,
    pub api_provider: ApiProvider,
    pub max_memory: usize,
    pub default_java: Option<Arc<JavaEntry>>,
    #[serde(default)]
    pub selected_java: Option<PathBuf>,
}

/// PCL theme
#[derive(Clone, serde::Serialize, serde::Deserialize)]
enum Theme {
    BlueLight,
    BlueDark,
}

/// PCL setup info default impl
impl Default for LauncherSetup {
    /// PCL setup info default impl
    fn default() -> Self {
        LauncherSetup {
            setup_version: constants::DEFAULT_JAVA_LIST_CACHE_VERSION,
            theme: Theme::BlueLight,
            api_provider: ApiProvider::Official,
            max_memory: 2048,
            default_java: None,
            selected_java: None,
        }
    }
}

/// Something that would be shared all over the app and may be displayed
#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct AppState {
    pub java_runtimes: Vec<crate::minecraft::java::JavaEntry>,
    pub accounts: Vec<McProfile>,
    #[serde(rename = "pcl_setup_info")]
    pub launcher_setup: LauncherSetup,
    pub active_account: Option<Arc<McProfile>>,
    pub repositories: Vec<McFolder>,
    pub active_game_instance: Option<Arc<McInstance>>,
    pub active_repo_path: PathBuf,
    #[serde(default)]
    pub java_preferences:
        HashMap<PathBuf, crate::minecraft::java::preference::InstanceJavaSettings>,
    #[serde(default)]
    pub migrated_java_ids: HashSet<String>,
    #[serde(default)]
    pub download_settings: Option<crate::io::download::resources::DownloadSettings>,
    #[serde(default)]
    pub log_max_lines: Option<u32>,
    #[serde(default)]
    pub window_settings: Option<WindowSettings>,
    #[serde(default)]
    pub memory_settings: Option<crate::minecraft::launch::memory::MemorySettings>,
    #[serde(default)]
    pub instance_memory_settings:
        HashMap<PathBuf, crate::minecraft::launch::memory::MemorySettings>,
    #[serde(default)]
    pub memory_custom: Option<crate::minecraft::launch::memory::MemoryCustom>,
    #[serde(default)]
    pub instance_memory_custom: HashMap<PathBuf, crate::minecraft::launch::memory::MemoryCustom>,
    #[serde(default)]
    pub migrated_memory_ids: HashSet<String>,
    #[serde(default)]
    pub migrated_memory_custom_ids: HashSet<String>,
    #[serde(default)]
    pub edited_memory_instances: HashSet<PathBuf>,
    #[serde(default)]
    pub isolation_default: crate::minecraft::instance::isolation::IsolationDefault,
    #[serde(default)]
    pub instance_isolation: HashMap<PathBuf, bool>,
    #[serde(default)]
    pub launch_arguments: Option<crate::minecraft::launch::arguments::LaunchArguments>,
    #[serde(default)]
    pub instance_launch_arguments:
        HashMap<PathBuf, crate::minecraft::launch::arguments::LaunchArguments>,
    #[serde(default)]
    pub launcher_visibility: crate::app::visibility::LauncherVisibility,
    #[serde(default)]
    pub instance_launch_settings:
        HashMap<PathBuf, crate::minecraft::launch::settings::InstanceLaunchSettings>,
    #[serde(default)]
    pub renderer: u8,
    #[serde(default = "crate::minecraft::launch::settings::default_version_info")]
    pub version_info: String,
    #[serde(default = "default_true")]
    pub disable_jlw: bool,
    #[serde(default)]
    pub disable_legacy_fix: bool,
    #[serde(default)]
    pub disable_lwjgl_unsafe_agent: bool,
    #[serde(default)]
    pub proxy: crate::io::net::proxy::ProxySettings,
    #[serde(default)]
    pub instance_descriptions: HashMap<PathBuf, String>,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WindowMode {
    Default,
    Launcher,
    Custom,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct WindowSettings {
    pub mode: WindowMode,
    pub width: u32,
    pub height: u32,
}

impl WindowSettings {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if !(100..=16384).contains(&self.width) || !(100..=16384).contains(&self.height) {
            return Err("Window dimensions must be between 100 and 16384 pixels".into());
        }
        Ok(())
    }
}
