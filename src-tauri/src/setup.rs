use tokio::sync::Mutex;

use crate::{
    core::{
        api_client::{ApiProvider, MinecraftApiClient},
        auth::Account,
        game::GameInstance,
        java::JavaRuntime,
        repository::GameRepository,
    },
    setup::constants::USER_AGENT,
    util,
};
use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::PathBuf,
    sync::{Arc, LazyLock},
};

pub mod constants {
    pub const DEFAULT_JAVA_LIST_CACHE_VERSION: i32 = 0;
    pub const LAUNCHER_NAME: &str = "PCL.Proto";
    pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const USER_AGENT: &str = concat!("PCL-Community/PCL.Proto/", env!("CARGO_PKG_VERSION"));
}

/// PCL global setup info, which user can modify
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PCLSetupInfo {
    setup_version: i32,
    theme: Theme,
    pub api_provider: ApiProvider,
    pub max_memory: usize,
    pub default_java: Option<Arc<JavaRuntime>>,
    #[serde(default)]
    pub selected_java: Option<PathBuf>,
}

/// PCL theme
#[derive(serde::Serialize, serde::Deserialize)]
enum Theme {
    BlueLight,
    BlueDark,
}

/// PCL setup info default impl
impl Default for PCLSetupInfo {
    /// PCL setup info default impl
    fn default() -> Self {
        PCLSetupInfo {
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
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct AppState {
    pub java_runtimes: Vec<crate::core::java::JavaRuntime>,
    pub accounts: Vec<Account>,
    pub pcl_setup_info: crate::setup::PCLSetupInfo,
    pub active_account: Option<Arc<Account>>,
    pub repositories: Vec<GameRepository>,
    pub active_game_instance: Option<Arc<GameInstance>>,
    pub active_repo_path: PathBuf,
    #[serde(default)]
    pub java_preferences: HashMap<PathBuf, crate::core::java::selection::InstanceJavaSettings>,
}

/// config manager, for loading and saving config file
/// should be Singleton
/// ## Fields
/// * `config_path` - the path of config file
pub struct ConfigManager {
    config_path: PathBuf,
    config_dir: PathBuf,
    pub app_state: Arc<Mutex<AppState>>,
    pub api_client: MinecraftApiClient,
    pub pcl_identifier: String,
}

#[derive(Debug)]
pub enum ConfigManagerError {
    ConfigDirNotFound,
    ConfigFileNotFound,
    ConfigFileCorrupted,
    IdentifierFailure,
    ApiClientBuildError,
}

pub static CONFIG_MANAGER: LazyLock<Option<ConfigManager>> = LazyLock::new(|| {
    let config_manager = ConfigManager::new();
    if let Ok(config_manager) = config_manager {
        Some(config_manager)
    } else {
        None
    }
});

impl ConfigManager {
    /// create a new config manager every time it launches
    /// and initialize the config file, for those who never use this launcher
    fn new() -> Result<Self, ConfigManagerError> {
        let config_dirs = directories::ProjectDirs::from("pcl-community", "pcl", "proto")
            .ok_or(ConfigManagerError::ConfigDirNotFound)?;
        let config_dir = config_dirs.config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).map_err(|_| ConfigManagerError::ConfigDirNotFound)?;
        }
        let config_path = config_dir.join("config.json");
        let identifier_path = config_dir.join("pcl_identifier.txt");
        let pcl_identifier = if identifier_path.exists() {
            fs::read_to_string(identifier_path)
                .map_err(|_| ConfigManagerError::IdentifierFailure)?
        } else {
            let board_serial = util::crypto::get_board_serial()
                .map_err(|_| ConfigManagerError::IdentifierFailure)?;
            let pcl_identifier = format!("{:x}", util::crypto::get_pcl_hash(&board_serial));
            fs::write(identifier_path, pcl_identifier.clone())
                .map_err(|_| ConfigManagerError::IdentifierFailure)?;
            pcl_identifier
        };
        let instance = Self {
            config_path,
            config_dir: config_dir.to_path_buf(),
            app_state: Arc::new(Mutex::new(AppState::default())),
            api_client: MinecraftApiClient::new(
                reqwest::Client::builder()
                    .user_agent(USER_AGENT)
                    .build()
                    .map_err(|_| ConfigManagerError::ApiClientBuildError)?,
                &ApiProvider::default(),
            ),
            pcl_identifier,
        };
        if !instance.config_path.exists()
            || !instance.config_path.is_file()
            || instance.load().is_err()
        {
            log::warn!("Unabled to load config file, try to init");
            instance.init()?;
            instance.save()?;
        }
        log::debug!("launched config at {:?}", instance.config_path);
        Ok(instance)
    }

    pub fn instance() -> &'static ConfigManager {
        CONFIG_MANAGER.as_ref().unwrap()
    }

    /// initialize the config file, for those who never use this launcher
    fn init(&self) -> Result<(), ConfigManagerError> {
        let game_dir = self.config_dir.join(".minecraft");
        if !game_dir.exists() {
            fs::create_dir_all(&game_dir).map_err(|_| ConfigManagerError::ConfigDirNotFound)?;
        }
        let mut state = self.app_state.blocking_lock();
        // [WARN] Only for Debug!!
        // TODO)) 后面去除下面的代码
        state.active_repo_path = game_dir.clone();
        state
            .repositories
            .push(GameRepository::new("Default", game_dir));
        // state.repositories.push(GameRepository::new(
        //     "HMCL",
        //     PathBuf::from("/Users/amagicpear/HMCL/.minecraft"),
        // ));
        state.active_account = Some(Arc::new(Account::Offline {
            username: "AMagicPear".to_string(),
            uuid: "31bbe5379fea4e68aa4ad7aacca23d13".to_string(),
        }));
        // state.pcl_setup_info.default_java =
        //     Some(Arc::new(JavaRuntime::try_from("/usr/bin/java").unwrap()));
        Ok(())
    }

    /// load the config file
    /// TODO: Partial config support
    fn load(&self) -> Result<(), ConfigManagerError> {
        let file = std::fs::File::open(&self.config_path)
            .map_err(|_| ConfigManagerError::ConfigFileNotFound)?;
        let mut reader = std::io::BufReader::new(file);
        let state_read: AppState = serde_json::from_reader(&mut reader)
            .map_err(|_| ConfigManagerError::ConfigFileCorrupted)?;
        let mut state = self.app_state.blocking_lock();
        *state = state_read;
        migrate_java_settings(&mut state);
        // update the api provider
        self.api_client
            .switch_provider(&state.pcl_setup_info.api_provider);
        Ok(())
    }

    /// save the config file
    pub fn save(&self) -> Result<(), ConfigManagerError> {
        static SAVE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _save = SAVE_LOCK
            .lock()
            .map_err(|_| ConfigManagerError::ConfigFileCorrupted)?;
        let state = self.app_state.blocking_lock();
        save_config_atomically(&self.config_path, &*state)
    }

    #[inline]
    pub fn identifier_path(&self) -> PathBuf {
        self.config_dir.join("pcl_identifier.txt")
    }

    pub fn java_runtime_dir(&self) -> PathBuf {
        self.config_dir.join("runtime")
    }
}

fn migrate_java_settings(state: &mut AppState) {
    // Legacy cached version numbers are untrusted; migrate only the executable path.
    if state.pcl_setup_info.selected_java.is_none() {
        state.pcl_setup_info.selected_java = state
            .pcl_setup_info
            .default_java
            .as_ref()
            .map(|j| j.java_exe.clone());
    }
    state.pcl_setup_info.default_java = None;
    if let Some(instance) = state.active_game_instance.as_ref()
        && let crate::core::game::GameJava::Custom(runtime) = &instance.game_java
    {
        let (path, java_exe) = (instance.directory.clone(), runtime.java_exe.clone());
        state.java_preferences.entry(path).or_insert_with(|| {
            crate::core::java::selection::InstanceJavaSettings {
                preference: crate::core::java::selection::JavaPreference::Existing { java_exe },
                ignore_compatibility: false,
            }
        });
    }
}

fn save_config_atomically(
    path: &std::path::Path,
    value: &impl serde::Serialize,
) -> Result<(), ConfigManagerError> {
    let temporary = path.with_extension(format!("{}.tmp", uuid::Uuid::new_v4()));
    let result = (|| {
        let file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|_| ConfigManagerError::ConfigFileNotFound)?;
        let mut writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, value)
            .map_err(|_| ConfigManagerError::ConfigFileCorrupted)?;
        writer
            .flush()
            .map_err(|_| ConfigManagerError::ConfigFileCorrupted)?;
        writer
            .get_ref()
            .sync_all()
            .map_err(|_| ConfigManagerError::ConfigFileCorrupted)?;
        drop(writer);
        fs::rename(&temporary, path).map_err(|_| ConfigManagerError::ConfigFileCorrupted)
    })();
    if result.is_err() {
        let _ = fs::remove_file(temporary);
    }
    result
}

#[test]
fn test_config_manager() {
    let config_manager = ConfigManager::instance();
    config_manager.load().unwrap();
    config_manager.save().unwrap();
}

#[cfg(test)]
mod java_config_tests {
    use super::*;
    use serde::ser::Error;
    #[test]
    fn failed_saves_preserve_previous_config_and_successful_saves_replace_it() {
        let root =
            std::env::temp_dir().join(format!("pcl-java-config-test-{}", uuid::Uuid::new_v4()));
        fs::create_dir(&root).unwrap();
        let path = root.join("config.json");
        fs::write(&path, b"previous config").unwrap();
        struct CannotSerialize;
        impl serde::Serialize for CannotSerialize {
            fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(S::Error::custom("fixture failure"))
            }
        }
        assert!(save_config_atomically(&path, &CannotSerialize).is_err());
        assert_eq!(fs::read(&path).unwrap(), b"previous config");
        let mut state = AppState::default();
        state.pcl_setup_info.selected_java = Some(PathBuf::from("/test/jdk/bin/java"));
        state.java_preferences.insert(
            PathBuf::from("/test/game"),
            crate::core::java::selection::InstanceJavaSettings {
                preference: crate::core::java::selection::JavaPreference::Auto,
                ignore_compatibility: true,
            },
        );
        save_config_atomically(&path, &state).unwrap();
        let restored: AppState = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        assert_eq!(
            restored.pcl_setup_info.selected_java,
            state.pcl_setup_info.selected_java
        );
        assert!(restored.java_preferences[&PathBuf::from("/test/game")].ignore_compatibility);
        assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn legacy_java_migration_preserves_only_path_and_does_not_override_new_preferences() {
        let mut data = serde_json::to_value(AppState::default()).unwrap();
        data["pcl_setup_info"]["default_java"] = serde_json::json!({
            "directory_path":"/legacy/bin","is_user_imported":false,"version":"25","slug_version":25,
            "is_64_bit":true,"architecture":"Arm64","compability":"Perfect","is_jdk":true,
            "java_exe":"/legacy/bin/java","implementor":"fixture"
        });
        let mut state: AppState = serde_json::from_value(data).unwrap();
        migrate_java_settings(&mut state);
        assert_eq!(
            state.pcl_setup_info.selected_java,
            Some(PathBuf::from("/legacy/bin/java"))
        );
        assert!(state.pcl_setup_info.default_java.is_none());
        migrate_java_settings(&mut state);
        assert_eq!(
            state.pcl_setup_info.selected_java,
            Some(PathBuf::from("/legacy/bin/java"))
        );
        state.pcl_setup_info.selected_java = Some(PathBuf::from("/new/bin/java"));
        state.pcl_setup_info.default_java = serde_json::from_value(serde_json::json!({
            "directory_path":"/legacy/bin","is_user_imported":false,"version":"25","slug_version":25,
            "is_64_bit":true,"architecture":"Arm64","compability":"Perfect","is_jdk":true,
            "java_exe":"/legacy/bin/java","implementor":"fixture"
        })).unwrap();
        migrate_java_settings(&mut state);
        assert_eq!(
            state.pcl_setup_info.selected_java,
            Some(PathBuf::from("/new/bin/java"))
        );
        assert!(state.pcl_setup_info.default_java.is_none());
    }
}
