use crate::{
    AppState,
    core::{
        api_client::{self, plugins::McPluginReport},
        auth::Account,
        game::{GameInstance, InstanceFolder},
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
pub async fn get_launch_arguments(
    instance_directory: Option<std::path::PathBuf>,
) -> Result<crate::core::launch_arguments::LaunchArguments, String> {
    let state = ConfigManager::instance().app_state.lock().await;
    if let Some(path) = instance_directory {
        current_instance(&state, &path).map_err(|error| error.to_string())?;
        Ok(state
            .instance_launch_arguments
            .get(&path)
            .cloned()
            .unwrap_or_default())
    } else {
        Ok(state
            .launch_arguments
            .clone()
            .unwrap_or_else(crate::core::launch_arguments::LaunchArguments::global_default))
    }
}

#[tauri::command]
pub async fn set_launch_arguments(
    instance_directory: Option<std::path::PathBuf>,
    settings: crate::core::launch_arguments::LaunchArguments,
) -> Result<crate::core::launch_arguments::LaunchArguments, String> {
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().update(|state| {
            crate::core::launch_arguments::update(state, instance_directory.as_deref(), settings)
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_isolation_default() -> crate::core::isolation::IsolationDefault {
    ConfigManager::instance()
        .app_state
        .lock()
        .await
        .isolation_default
}

#[tauri::command]
pub async fn set_isolation_default(
    value: crate::core::isolation::IsolationDefault,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().update(|state| crate::core::isolation::set_default(state, value))
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_instance_isolation(
    instance_directory: std::path::PathBuf,
) -> Result<bool, String> {
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().ensure_instance_isolation(&instance_directory)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn set_instance_isolation(
    instance_directory: std::path::PathBuf,
    value: bool,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().update(|state| {
            current_instance(state, &instance_directory).map_err(|error| error.to_string())?;
            state.instance_isolation.insert(instance_directory, value);
            Ok(())
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_memory_settings(
    instance_directory: Option<std::path::PathBuf>,
) -> Result<crate::core::memory::MemorySnapshot, String> {
    use crate::core::memory::{MemoryCustom, MemorySettings, MemorySnapshot};
    let state = ConfigManager::instance().app_state.lock().await;
    match instance_directory {
        Some(directory) => {
            current_instance(&state, &directory).map_err(|error| error.to_string())?;
            let settings = state.instance_memory_settings.get(&directory).cloned();
            let custom = state
                .instance_memory_custom
                .get(&directory)
                .cloned()
                .unwrap_or_else(|| MemoryCustom::from_settings(settings.as_ref()));
            Ok(MemorySnapshot { settings, custom })
        }
        None => {
            let settings = Some(
                state
                    .memory_settings
                    .clone()
                    .unwrap_or(MemorySettings::Custom {
                        maximum_mib: u32::try_from(state.pcl_setup_info.max_memory)
                            .map_err(|error| error.to_string())?,
                        initial_mib: None,
                    }),
            );
            let custom = state
                .memory_custom
                .clone()
                .unwrap_or_else(|| MemoryCustom::from_settings(settings.as_ref()));
            Ok(MemorySnapshot { settings, custom })
        }
    }
}

#[tauri::command]
pub async fn set_memory_settings(
    instance_directory: Option<std::path::PathBuf>,
    settings: Option<crate::core::memory::MemorySettings>,
    custom: Option<crate::core::memory::MemoryCustom>,
    migrate_only: bool,
) -> Result<crate::core::memory::MemorySnapshot, String> {
    use crate::core::memory::{MemoryCustom, MemorySnapshot};
    if let Some(settings) = &settings {
        settings.validate()?;
    }
    if let Some(custom) = &custom {
        custom.validate()?;
    }
    if let (
        Some(crate::core::memory::MemorySettings::Custom {
            maximum_mib,
            initial_mib,
        }),
        Some(custom),
    ) = (&settings, &custom)
    {
        if *maximum_mib != custom.maximum_mib
            || *initial_mib
                != custom
                    .initial_mib
                    .map(|value| value.min(custom.maximum_mib))
        {
            return Err("Custom memory values do not match the active settings".into());
        }
    }
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().update(|state| {
            if let Some(directory) = instance_directory {
                current_instance(state, &directory).map_err(|error| error.to_string())?;
                if migrate_only && let Some(saved) = state.instance_memory_settings.get(&directory)
                {
                    return Ok(MemorySnapshot {
                        settings: Some(saved.clone()),
                        custom: state
                            .instance_memory_custom
                            .get(&directory)
                            .cloned()
                            .unwrap_or_else(|| MemoryCustom::from_settings(Some(saved))),
                    });
                }
                if let Some(settings) = settings.clone() {
                    state
                        .instance_memory_settings
                        .insert(directory.clone(), settings);
                } else {
                    state.instance_memory_settings.remove(&directory);
                }
                let custom =
                    custom.unwrap_or_else(|| MemoryCustom::from_settings(settings.as_ref()));
                state
                    .instance_memory_custom
                    .insert(directory.clone(), custom.clone());
                state.edited_memory_instances.insert(directory);
                Ok(MemorySnapshot { settings, custom })
            } else {
                let value = settings
                    .clone()
                    .ok_or("Global memory settings cannot inherit")?;
                if migrate_only && let Some(saved) = &state.memory_settings {
                    let saved = saved.clone();
                    if state.memory_custom.is_none() {
                        state.memory_custom = match &saved {
                            crate::core::memory::MemorySettings::Custom { .. } => {
                                Some(MemoryCustom::from_settings(Some(&saved)))
                            }
                            _ => custom,
                        };
                    }
                    let custom = state
                        .memory_custom
                        .clone()
                        .unwrap_or_else(|| MemoryCustom::from_settings(Some(&saved)));
                    return Ok(MemorySnapshot {
                        settings: Some(saved),
                        custom,
                    });
                }
                state.memory_settings = Some(value.clone());
                let custom = custom.unwrap_or_else(|| MemoryCustom::from_settings(Some(&value)));
                state.memory_custom = Some(custom.clone());
                Ok(MemorySnapshot {
                    settings: Some(value),
                    custom,
                })
            }
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn migrate_instance_memory_settings(
    legacy: std::collections::HashMap<String, crate::core::memory::MemorySnapshot>,
) -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().migrate_instance_memory_settings(legacy)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[derive(serde::Serialize)]
pub struct MemoryPreview {
    total_bytes: u64,
    available_bytes: u64,
    allocation_mib: Option<usize>,
}

#[tauri::command]
pub async fn preview_memory_settings(
    instance_directory: Option<std::path::PathBuf>,
    settings: Option<crate::core::memory::MemorySettings>,
    is_64_bit: Option<bool>,
) -> Result<MemoryPreview, String> {
    tauri::async_runtime::spawn_blocking(move || {
        use crate::core::memory::{self, MemorySettings};
        let config = ConfigManager::instance();
        let path = {
            let state = config.app_state.blocking_lock();
            instance_directory.clone().or_else(|| {
                state
                    .active_game_instance
                    .as_ref()
                    .map(|instance| instance.directory.clone())
            })
        };
        if let Some(path) = &path {
            config.ensure_instance_isolation(path)?;
        }
        let (instance, effective) = {
            let state = config.app_state.blocking_lock();
            let instance = if let Some(path) = &instance_directory {
                Some(
                    current_instance(&state, path)
                        .map_err(|error| error.to_string())?
                        .clone(),
                )
            } else {
                state.active_game_instance.as_deref().cloned()
            };
            let effective = settings
                .or_else(|| state.memory_settings.clone())
                .unwrap_or(MemorySettings::Custom {
                    maximum_mib: u32::try_from(state.pcl_setup_info.max_memory)
                        .map_err(|error| error.to_string())?,
                    initial_mib: None,
                });
            let game_directory = instance
                .as_ref()
                .map(|instance| crate::core::isolation::directory(&state, instance).to_owned());
            (
                instance.map(|instance| (instance, game_directory.unwrap())),
                effective,
            )
        };
        effective.validate()?;
        let mut system = sysinfo::System::new();
        system.refresh_memory();
        let total = system.total_memory();
        if total == 0 {
            return Err("Physical memory is unavailable".into());
        }
        let available = system.available_memory().min(total);
        let allocation = match is_64_bit {
            None => None,
            Some(is_64_bit) => {
                let value = if let Some((instance, game_directory)) = &instance {
                    memory::resolve(&effective, available, instance, game_directory, is_64_bit)?.0
                } else {
                    let max = match effective {
                        MemorySettings::Auto => memory::automatic(available, false, false, 0)?,
                        MemorySettings::Custom { maximum_mib, .. } => maximum_mib as usize,
                    };
                    if is_64_bit { max } else { max.min(1024) }
                };
                Some(value)
            }
        };
        Ok(MemoryPreview {
            total_bytes: total,
            available_bytes: available,
            allocation_mib: allocation,
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn launch_game(
    window: tauri::WebviewWindow,
    instance_directory: std::path::PathBuf,
) -> Result<(), String> {
    let info = java::selection::prepare(instance_directory.clone()).await?;
    let runtime = info.runtime.ok_or_else(|| {
        format!(
            "No compatible Java found: {}",
            info.requirements.description()
        )
    })?;
    let expected = instance_directory.clone();
    let mut option = tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().ensure_instance_isolation(&instance_directory)?;
        let state = ConfigManager::instance().app_state.blocking_lock();
        current_instance(&state, &instance_directory).map_err(|e| e.to_string())?;
        let option =
            LaunchOption::from_state(&state, Arc::new(runtime)).map_err(|e| e.to_string())?;
        Ok::<_, String>(option)
    })
    .await
    .map_err(|e| e.to_string())??;
    if option.uses_launcher_size() {
        let size = window.inner_size().map_err(|error| error.to_string())?;
        if size.width == 0 || size.height == 0 {
            return Err("Launcher window size is unavailable".into());
        }
        option = option.with_window_size(size.width as usize, size.height as usize);
    }
    let config = ConfigManager::instance();
    let downloader =
        crate::core::downloader::Downloader::with_bases(config.api_client.api_bases_async().await);
    let natives = option
        .prepare_natives(&downloader)
        .await
        .map_err(|e| format!("{e:#}"))?;
    option
        .prepare_legacy_assets(&downloader)
        .await
        .map_err(|e| format!("{e:#}"))?;
    tauri::async_runtime::spawn_blocking(move || {
        let state = config.app_state.blocking_lock();
        current_instance(&state, &expected).map_err(|e| e.to_string())?;
        let visibility = state.launcher_visibility;
        let child = option.launch_checked().map_err(|e| e.to_string())?;
        drop(state);
        crate::core::launch_session::track(
            child,
            expected,
            option.access_token(),
            move || drop(natives),
            move |id, event| crate::core::visibility::handle(&window, id, visibility, event),
        );
        Ok(())
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
    let runtime = tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().update(|state| {
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
            Ok(runtime)
        })
    })
    .await
    .map_err(|e| e.to_string())??;
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
        Some(actual.java_exe)
    } else {
        None
    };
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().update(|state| {
            if let Some(path) = &selected {
                let saved = state
                    .java_runtimes
                    .iter()
                    .find(|j| &j.java_exe == path)
                    .ok_or("Java is unavailable; refresh the list")?;
                if !saved.is_enabled {
                    return Err("Enable this Java before selecting it".into());
                }
            }
            state.pcl_setup_info.selected_java = selected;
            Ok(())
        })
    })
    .await
    .map_err(|e| e.to_string())?
}
#[tauri::command]
pub async fn set_java_enabled(java_exe: std::path::PathBuf, enabled: bool) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().update(|state| {
            if !enabled && state.pcl_setup_info.selected_java.as_ref() == Some(&java_exe) {
                return Err("Deselect this Java as the global default before disabling it".into());
            }
            let runtime = state
                .java_runtimes
                .iter_mut()
                .find(|j| j.java_exe == java_exe)
                .ok_or("Java is unavailable; refresh the list")?;
            runtime.is_enabled = enabled;
            Ok(())
        })
    })
    .await
    .map_err(|e| e.to_string())?
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
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().update(|state| {
            current_instance(&state, &instance_directory).map_err(|e| e.to_string())?;
            state.java_preferences.insert(instance_directory, settings);
            Ok(())
        })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn resolve_legacy_instance_ids(
    ids: std::collections::HashSet<String>,
) -> Result<std::collections::HashMap<String, std::path::PathBuf>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().resolve_legacy_instance_ids(ids)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn migrate_instance_java_settings(
    legacy: std::collections::HashMap<String, InstanceJavaSettings>,
) -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().migrate_instance_java_settings(legacy)
    })
    .await
    .map_err(|error| error.to_string())?
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
        open_directory(&runtime.directory_path)
    })
    .await
    .map_err(|e| e.to_string())?
}

fn open_directory(path: &std::path::Path) -> Result<(), String> {
    let mut command = std::process::Command::new(if cfg!(windows) {
        "explorer"
    } else if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    });
    command
        .arg(path)
        .spawn()
        .map_err(|error| error.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn open_instance_folder(
    instance_directory: std::path::PathBuf,
    kind: InstanceFolder,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().ensure_instance_isolation(&instance_directory)?;
        let state = ConfigManager::instance().app_state.blocking_lock();
        let instance =
            current_instance(&state, &instance_directory).map_err(|error| error.to_string())?;
        let game_directory = crate::core::isolation::directory(&state, instance);
        let directory =
            crate::core::game::prepare_shortcut_directory(instance, game_directory, kind)
                .map_err(|error| error.to_string())?;
        open_directory(&directory)
    })
    .await
    .map_err(|error| error.to_string())?
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
    let runtime = tauri::async_runtime::spawn_blocking(move || {
        config.update(|state| {
            state.java_runtimes.push(runtime.clone());
            java::sort_list(&mut state.java_runtimes);
            Ok(runtime)
        })
    })
    .await
    .map_err(|e| e.to_string())??;
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
pub async fn select_repository(repository_index: usize) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().select_repository(repository_index)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command(rename_all = "snake_case")]
pub fn add_new_repository(
    new_repo_path: &str,
    new_repo_name: &str,
) -> Result<Vec<GameRepository>, String> {
    ConfigManager::instance().add_repository(new_repo_name, std::path::Path::new(new_repo_path))
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
pub async fn instance_folder_action(
    repository_index: usize,
    instance_name: String,
    instance_directory: std::path::PathBuf,
    delete: bool,
) -> Result<(), String> {
    crate::util::file::validate_file_name(&instance_name).map_err(|e| e.to_string())?;
    tauri::async_runtime::spawn_blocking(move || {
        let manager = ConfigManager::instance();
        let repo = manager
            .app_state
            .blocking_lock()
            .repositories
            .get(repository_index)
            .cloned()
            .ok_or("Minecraft folder index is out of range")?;
        let instance = repo
            .game_instance_named(&instance_name)
            .ok_or("Minecraft instance was not found")?;
        if instance.directory != instance_directory {
            return Err("Minecraft folder changed; try again".into());
        }
        let versions = repo
            .path
            .join("versions")
            .canonicalize()
            .map_err(|e| e.to_string())?;
        let path = instance
            .directory
            .canonicalize()
            .map_err(|e| e.to_string())?;
        if path.parent() != Some(versions.as_path())
            || std::fs::symlink_metadata(&instance.directory)
                .map_err(|e| e.to_string())?
                .file_type()
                .is_symlink()
        {
            return Err("Instance directory is not a direct, canonical child of versions".into());
        }
        if !delete {
            return open_directory(&path);
        }
        trash::delete(&path).map_err(|e| e.to_string())?;
        manager.update(|state| {
            if state
                .active_game_instance
                .as_ref()
                .is_some_and(|active| active.directory == instance.directory)
            {
                state.active_game_instance = None;
            }
            Ok(())
        })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_active_instance(
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<Option<GameInstance>, String> {
    let Some(active_instance) = state.lock().await.active_game_instance.clone() else {
        return Ok(None);
    };
    tauri::async_runtime::spawn_blocking(move || {
        active_instance
            .refresh()
            .unwrap_or_else(|_| (*active_instance).clone())
    })
    .await
    .map(Some)
    .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn select_instance(
    state: State<'_, Arc<Mutex<AppState>>>,
    repository_index: usize,
    instance_name: String,
) -> Result<GameInstance, String> {
    crate::util::file::validate_file_name(&instance_name).map_err(|error| error.to_string())?;
    let repo = state
        .lock()
        .await
        .repositories
        .get(repository_index)
        .cloned()
        .ok_or("Minecraft folder index is out of range")?;
    let repo_path = repo.path.clone();
    let instance =
        tauri::async_runtime::spawn_blocking(move || repo.game_instance_named(&instance_name))
            .await
            .map_err(|error| error.to_string())?
            .ok_or("Minecraft instance was not found")?;
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().select_instance(
            repository_index,
            &repo_path,
            instance.clone(),
        )?;
        Ok(instance)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_version_manifest(
    client: State<'_, &api_client::MinecraftApiClient>,
) -> Result<api_client::game::VersionManifest, String> {
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
        mcmod::PluginType::Fabric => {
            api_client
                .get_fabric_versions(mc_version, api_client::FabricFamily::Fabric)
                .await
        }
        mcmod::PluginType::LegacyFabric => {
            api_client
                .get_fabric_versions(mc_version, api_client::FabricFamily::Legacyfabric)
                .await
        }
        _ => Err(api_client::McApiError::PluginMismatch(plugin_type)),
    };
    verisons.map_err(|err| err.to_string())
}
