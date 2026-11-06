use crate::config::{current_instance, manager::ConfigManager};

#[tauri::command]
pub async fn get_launch_arguments(
    instance_directory: Option<std::path::PathBuf>,
) -> Result<pcl_core::minecraft::launch::arguments::LaunchArguments, String> {
    let state = ConfigManager::instance().app_state.lock().await;
    if let Some(path) = instance_directory {
        current_instance(&state, &path).map_err(|error| error.to_string())?;
        Ok(state
            .instance_launch_arguments
            .get(&path)
            .cloned()
            .unwrap_or_default())
    } else {
        Ok(state.launch_arguments.clone().unwrap_or_else(
            pcl_core::minecraft::launch::arguments::LaunchArguments::global_default,
        ))
    }
}

#[tauri::command]
pub async fn set_launch_arguments(
    instance_directory: Option<std::path::PathBuf>,
    settings: pcl_core::minecraft::launch::arguments::LaunchArguments,
) -> Result<pcl_core::minecraft::launch::arguments::LaunchArguments, String> {
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().update(|state| {
            crate::config::arguments::update(state, instance_directory.as_deref(), settings)
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_instance_launch_settings(
    instance_directory: std::path::PathBuf,
) -> Result<pcl_core::minecraft::launch::settings::InstanceLaunchSettings, String> {
    let state = ConfigManager::instance().app_state.lock().await;
    current_instance(&state, &instance_directory).map_err(|error| error.to_string())?;
    Ok(state
        .instance_launch_settings
        .get(&instance_directory)
        .cloned()
        .unwrap_or_default())
}

#[tauri::command]
pub async fn set_instance_launch_settings(
    instance_directory: std::path::PathBuf,
    settings: pcl_core::minecraft::launch::settings::InstanceLaunchSettings,
) -> Result<pcl_core::minecraft::launch::settings::InstanceLaunchSettings, String> {
    tauri::async_runtime::spawn_blocking(move || {
        ConfigManager::instance().update(|state| {
            crate::config::launch_settings::update(state, &instance_directory, settings)
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

#[cfg(test)]
mod tests {
    /// 前端表单默认值必须与核心的默认 JVM 参数保持一致。
    #[test]
    fn ui_defaults_match_core_jvm_arguments() {
        let ui: serde_json::Value =
            serde_json::from_str(include_str!("../../../src/assets/setting-defaults.json"))
                .unwrap();
        assert_eq!(
            ui["PageSetupLaunch"]["TextAdvanceJvm"],
            pcl_core::minecraft::launch::arguments::DEFAULT_JVM
        );
    }
}
