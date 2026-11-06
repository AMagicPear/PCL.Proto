//! 启动器设置类命令：窗口、可见性、版本说明和日志行数。

use pcl_core::app::config::{self, WindowSettings};
use pcl_core::app::visibility::LauncherVisibility;

#[tauri::command]
pub async fn get_window_settings() -> Option<WindowSettings> {
    config::get_window_settings().await
}

#[tauri::command]
pub async fn set_window_settings(settings: WindowSettings) -> Result<WindowSettings, String> {
    config::set_window_settings(settings).await
}

#[tauri::command]
pub async fn get_launcher_visibility() -> LauncherVisibility {
    config::get_launcher_visibility().await
}

#[tauri::command]
pub async fn set_launcher_visibility(mode: LauncherVisibility) -> Result<(), String> {
    config::set_launcher_visibility(mode).await
}

#[tauri::command]
pub async fn get_version_info() -> String {
    config::get_version_info().await
}

#[tauri::command]
pub async fn set_version_info(value: String) -> Result<String, String> {
    config::set_version_info(value).await
}

#[tauri::command]
pub async fn get_log_max_lines() -> Option<u32> {
    config::get_log_max_lines().await
}

#[tauri::command]
pub async fn set_log_max_lines(lines: u32) -> Result<u32, String> {
    config::set_log_max_lines(lines).await
}
