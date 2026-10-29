use crate::{
    AppState,
    core::{
        api_client::{self, plugins::McPluginReport},
        auth::Account,
        game::GameInstance,
        java::{
            self, JavaRuntime,
            selection::{InstanceJavaSettings, JavaLaunchInfo, JavaPreference, current_instance},
        },
        launcher::LaunchOption,
        mcmod,
        repository::GameRepository,
    },
    setup::ConfigManager,
};
use std::sync::Arc;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn launch_game(instance_directory: std::path::PathBuf) -> Result<(), String> {
    let info = java::selection::prepare(instance_directory.clone()).await?;
    let runtime = info.runtime.ok_or_else(|| {
        format!(
            "No compatible Java found: {}",
            info.requirements.description()
        )
    })?;
    tauri::async_runtime::spawn_blocking(move || {
        let state = ConfigManager::instance().app_state.blocking_lock();
        current_instance(&state, &instance_directory).map_err(|e| e.to_string())?;
        let option =
            LaunchOption::from_state(&state, Arc::new(runtime)).map_err(|e| e.to_string())?;
        drop(state);
        option.launch_checked().map_err(|e| e.to_string())?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

async fn save_java_config() -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(|| {
        ConfigManager::instance()
            .save()
            .map_err(|e| format!("{e:?}"))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn add_java(app: AppHandle) -> Result<Option<JavaRuntime>, String> {
    let picked =
        tauri::async_runtime::spawn_blocking(move || app.dialog().file().blocking_pick_file())
            .await
            .map_err(|e| e.to_string())?;
    let Some(file) = picked else {
        return Ok(None);
    };
    let path = file.into_path().map_err(|e| e.to_string())?;
    let mut runtime = tauri::async_runtime::spawn_blocking(move || JavaRuntime::probe(&path))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    runtime.is_user_imported = true;
    {
        let mut state = ConfigManager::instance().app_state.lock().await;
        if let Some(existing) = state
            .java_runtimes
            .iter_mut()
            .find(|j| j.java_exe == runtime.java_exe)
        {
            existing.is_user_imported = true;
            runtime = existing.clone();
        } else {
            state.java_runtimes.push(runtime.clone());
            java::sort_list(&mut state.java_runtimes);
        }
    }
    save_java_config().await?;
    Ok(Some(runtime))
}

#[tauri::command]
pub async fn get_java_list(
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<Vec<JavaRuntime>, String> {
    Ok(state.lock().await.java_runtimes.clone())
}

#[tauri::command]
pub async fn refresh_java_list() -> Result<Vec<JavaRuntime>, String> {
    java::refresh(true).await
}

#[derive(serde::Serialize)]
pub struct JavaSettingsInfo {
    global_java: Option<std::path::PathBuf>,
    instance: InstanceJavaSettings,
    has_instance_settings: bool,
    global_is_64_bit: bool,
    instance_is_64_bit: bool,
}
#[tauri::command]
pub async fn get_java_settings(
    instance_directory: Option<std::path::PathBuf>,
) -> Result<JavaSettingsInfo, String> {
    let state = ConfigManager::instance().app_state.lock().await;
    let has_instance_settings = instance_directory
        .as_ref()
        .is_some_and(|d| state.java_preferences.contains_key(d));
    let instance = if let Some(directory) = instance_directory {
        current_instance(&state, &directory).map_err(|e| e.to_string())?;
        state
            .java_preferences
            .get(&directory)
            .cloned()
            .unwrap_or_default()
    } else {
        InstanceJavaSettings::default()
    };
    let runtimes = state.java_runtimes.clone();
    let global_java = state.pcl_setup_info.selected_java.clone();
    drop(state);
    tauri::async_runtime::spawn_blocking(move || {
        let global_is_64_bit = java::selection::is_game_set_64_bit(
            &runtimes,
            global_java.as_deref(),
            &JavaPreference::Global,
        );
        let instance_is_64_bit = java::selection::is_game_set_64_bit(
            &runtimes,
            global_java.as_deref(),
            &instance.preference,
        );
        JavaSettingsInfo {
            global_java,
            instance,
            has_instance_settings,
            global_is_64_bit,
            instance_is_64_bit,
        }
    })
    .await
    .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn set_global_java(java_exe: Option<std::path::PathBuf>) -> Result<(), String> {
    let selected = if let Some(path) = java_exe {
        let actual = tauri::async_runtime::spawn_blocking(move || JavaRuntime::probe(&path))
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?;
        let state = ConfigManager::instance().app_state.lock().await;
        let saved = state
            .java_runtimes
            .iter()
            .find(|j| j.java_exe == actual.java_exe)
            .ok_or("Java is unavailable; refresh the list")?;
        if !saved.is_enabled {
            return Err("Enable this Java before selecting it".into());
        }
        Some(actual.java_exe)
    } else {
        None
    };
    ConfigManager::instance()
        .app_state
        .lock()
        .await
        .pcl_setup_info
        .selected_java = selected;
    save_java_config().await
}
#[tauri::command]
pub async fn set_java_enabled(java_exe: std::path::PathBuf, enabled: bool) -> Result<(), String> {
    {
        let mut state = ConfigManager::instance().app_state.lock().await;
        if !enabled && state.pcl_setup_info.selected_java.as_ref() == Some(&java_exe) {
            return Err("Deselect this Java as the global default before disabling it".into());
        }
        let runtime = state
            .java_runtimes
            .iter_mut()
            .find(|j| j.java_exe == java_exe)
            .ok_or("Java is unavailable; refresh the list")?;
        runtime.is_enabled = enabled;
    }
    save_java_config().await
}
#[tauri::command]
pub async fn set_instance_java_settings(
    instance_directory: std::path::PathBuf,
    settings: InstanceJavaSettings,
) -> Result<(), String> {
    let validated = settings.clone();
    tauri::async_runtime::spawn_blocking(move || {
        java::selection::validate_preference(&validated.preference)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())?;
    {
        let mut state = ConfigManager::instance().app_state.lock().await;
        current_instance(&state, &instance_directory).map_err(|e| e.to_string())?;
        state.java_preferences.insert(instance_directory, settings);
    }
    save_java_config().await
}
#[tauri::command]
pub async fn select_relative_java(app: AppHandle) -> Result<Option<JavaPreference>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let root = java::launcher_directory().map_err(|e| e.to_string())?;
        let Some(file) = app
            .dialog()
            .file()
            .set_directory(&root)
            .blocking_pick_file()
        else {
            return Ok(None);
        };
        let path = file
            .into_path()
            .map_err(|e| e.to_string())?
            .canonicalize()
            .map_err(|e| e.to_string())?;
        let root = root.canonicalize().map_err(|e| e.to_string())?;
        let relative = path
            .strip_prefix(&root)
            .map_err(|_| "Java path is outside the launcher directory")?
            .to_owned();
        java::selection::relative_java(&root, &relative).map_err(|e| e.to_string())?;
        JavaRuntime::probe(&path).map_err(|e| e.to_string())?;
        Ok(Some(JavaPreference::Relative {
            relative_path: relative,
        }))
    })
    .await
    .map_err(|e| e.to_string())?
}
#[tauri::command]
pub async fn open_java_folder(java_exe: std::path::PathBuf) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let runtime = JavaRuntime::probe(&java_exe).map_err(|e| e.to_string())?;
        let mut command = std::process::Command::new(if cfg!(windows) {
            "explorer"
        } else if cfg!(target_os = "macos") {
            "open"
        } else {
            "xdg-open"
        });
        command
            .arg(runtime.directory_path)
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}
#[tauri::command]
pub async fn prepare_game_java(
    instance_directory: std::path::PathBuf,
) -> Result<JavaLaunchInfo, String> {
    java::selection::prepare(instance_directory).await
}

static JAVA_INSTALL: Mutex<Option<tokio::task::AbortHandle>> = Mutex::const_new(None);
#[tauri::command]
pub async fn download_game_java(
    instance_directory: std::path::PathBuf,
    progress: tauri::ipc::Channel<java::install::JavaInstallProgress>,
) -> Result<JavaRuntime, String> {
    let mut active = JAVA_INSTALL.lock().await;
    if active.is_some() {
        return Err("A Java download is already running".into());
    }
    let info = java::selection::prepare(instance_directory).await?;
    let config = ConfigManager::instance();
    let bases = config.api_client.api_bases_async().await;
    let task = tauri::async_runtime::spawn(java::install::install(
        info.requirements,
        config.java_runtime_dir(),
        bases,
        progress,
    ));
    *active = Some(task.inner().abort_handle());
    drop(active);
    let result = task
        .await
        .map_err(|e| e.to_string())
        .and_then(|r| r.map_err(|e| e.to_string()));
    *JAVA_INSTALL.lock().await = None;
    let runtime = result?;
    {
        let mut state = config.app_state.lock().await;
        state.java_runtimes.push(runtime.clone());
        java::sort_list(&mut state.java_runtimes);
    }
    save_java_config().await?;
    Ok(runtime)
}
#[tauri::command]
pub async fn cancel_java_download() {
    if let Some(task) = JAVA_INSTALL.lock().await.as_ref() {
        task.abort();
    }
}

#[tauri::command]
pub fn get_repositories(state: State<'_, Arc<Mutex<AppState>>>) -> Vec<GameRepository> {
    let state = state.blocking_lock();
    state.repositories.clone()
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_repository(
    state: State<'_, Arc<Mutex<AppState>>>,
    repository_index: usize,
) -> Result<(), String> {
    let mut state = state.lock().await;
    let path = state
        .repositories
        .get(repository_index)
        .ok_or("Minecraft folder index is out of range")?
        .path
        .clone();
    state.active_repo_path = path;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn add_new_repository(
    state: State<'_, Arc<Mutex<AppState>>>,
    new_repo_path: &str,
    new_repo_name: &str,
) -> Result<Vec<GameRepository>, String> {
    let mut guard = state.blocking_lock();
    if guard.repositories.iter().any(|repo_old| {
        repo_old.name == new_repo_name || repo_old.path.as_os_str() == new_repo_path
    }) {
        return Err("repo info conflits with current!".into());
    }
    guard
        .repositories
        .push(GameRepository::new(new_repo_name, new_repo_path.into()));
    log::info!(
        "added new repo, name: {}, path: {}, repos count now: {}",
        new_repo_name,
        new_repo_path,
        guard.repositories.len()
    );
    Ok(guard.repositories.clone())
}

/// async commands that contain references as inputs must return a Result
/// so this place returns an option
#[tauri::command]
pub async fn get_account() -> Option<Account> {
    let account = &ConfigManager::instance()
        .app_state
        .lock()
        .await
        .active_account;
    account.as_deref().cloned()
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_instances_in_repository(
    state: State<'_, Arc<Mutex<AppState>>>,
    repository_index: usize,
) -> Result<Vec<GameInstance>, String> {
    let repo = state
        .lock()
        .await
        .repositories
        .get(repository_index)
        .cloned()
        .ok_or("Minecraft folder index is out of range")?;
    tauri::async_runtime::spawn_blocking(move || repo.game_instances())
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_active_instance(state: State<'_, Arc<Mutex<AppState>>>) -> Option<GameInstance> {
    let guard = state.blocking_lock();
    let active_instance = guard.active_game_instance.clone();
    active_instance.as_deref().cloned()
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_instance(
    state: State<'_, Arc<Mutex<AppState>>>,
    repository_index: usize,
    instance_id: String,
) -> Result<(), String> {
    let repo = state
        .lock()
        .await
        .repositories
        .get(repository_index)
        .cloned()
        .ok_or("Minecraft folder index is out of range")?;
    let instances = tauri::async_runtime::spawn_blocking(move || repo.game_instances())
        .await
        .map_err(|error| error.to_string())?;
    let instance = instances
        .into_iter()
        .find(|instance| instance.id == instance_id)
        .ok_or("Minecraft instance was not found")?;
    let mut guard = state.lock().await;
    guard.active_repo_path = instance.global_dir.clone();
    guard.active_game_instance = Some(Arc::new(instance));
    Ok(())
}

#[tauri::command]
pub async fn get_version_manifest(
    client: State<'_, &api_client::MinecraftApiClient>,
    download_settings: Option<crate::core::downloader::resources::DownloadSettings>,
) -> Result<api_client::game::VersionManifest, String> {
    if let Some(settings) = download_settings {
        crate::core::downloader::resources::DOWNLOAD_RESOURCES.configure(settings)?;
    }
    match client
        .get_with_endpoint::<api_client::game::VersionManifest>(
            api_client::game::VERSION_MANIFEST_ENDPOINT,
            true,
        )
        .await
    {
        Ok(manifest) => Ok(manifest),
        Err(e) => return Err(format!("get_version_manifest: {:?}", e)),
    }
}

/// return the version json
#[tauri::command]
pub async fn handle_clicked_on_version(
    client: State<'_, &api_client::MinecraftApiClient>,
    id: &str,
) -> Result<api_client::game::VersionDetails, String> {
    let temp_dir = std::env::temp_dir().join(format!("pcl-proto-{}", id));
    log::debug!("selected tmp: {:?}", &temp_dir);
    let version_detail = client
        .get_version_details(id, &temp_dir)
        .await
        .map_err(|err| err.to_string())?;
    Ok(version_detail)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_plugin_versions(
    api_client: State<'_, &api_client::MinecraftApiClient>,
    plugin_type: mcmod::PluginType,
    mc_version: &str,
) -> Result<Vec<McPluginReport>, String> {
    let verisons = match plugin_type {
        mcmod::PluginType::Forge => api_client.get_forge_versions(mc_version).await,
        mcmod::PluginType::Fabric => api_client.get_fabric_versions(mc_version).await,
        _ => Err(api_client::McApiError::PluginMismatch(plugin_type)),
    };
    verisons.map_err(|err| err.to_string())
}
