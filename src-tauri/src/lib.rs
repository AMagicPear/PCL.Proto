//  ____   ____ _       ____            _
// |  _ \ / ___| |     |  _ \ _ __ ___ | |_ ___
// | |_) | |   | |     | |_) | '__/ _ \| __/ _ \
// |  __/| |___| |___ _|  __/| | | (_) | || (_) |
// |_|    \____|_____(_)_|   |_|  \___/ \__\___/
//
use setup::AppState;
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_log::{Target, TargetKind};

mod commands;
mod core;
mod setup;
mod util;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    // Target::new(TargetKind::Webview),
                ])
                .level(log::LevelFilter::Info)
                .build(),
        )
        .setup(|app| {
            if let Some(config_manager) = setup::CONFIG_MANAGER.as_ref() {
                app.manage(Arc::clone(&config_manager.app_state));
                app.manage(&config_manager.api_client);
                terracotta::init_lib(config_manager.identifier_path());
            } else {
                log::error!("CONFIG_MANAGER is None");
                app.dialog()
                    .message("Config manager failed to initialize!")
                    .title("Fatal Error! 完🥚辣！")
                    .buttons(tauri_plugin_dialog::MessageDialogButtons::Ok)
                    .show(|_result| {
                        std::process::exit(1);
                    });
            }
            // let easytier_manager = easytier::instance_manager::NetworkInstanceManager::new();
            // app.manage(easytier_manager);
            // let window = app.get_webview_window("main").unwrap();
            // window.on_navigation(move |url| {});
            // search for Java during init
            tauri::async_runtime::spawn(async move {
                if let Err(error) = core::java::refresh(false).await {
                    log::error!("Java scan failed: {error}");
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::launch_game,
            core::downloader::resources::get_download_stats,
            core::downloader::resources::set_download_settings,
            util::toys::get_lucky_today,
            commands::add_java,
            commands::get_java_list,
            commands::refresh_java_list,
            commands::get_java_settings,
            commands::set_global_java,
            commands::set_java_enabled,
            commands::set_instance_java_settings,
            commands::select_relative_java,
            commands::open_java_folder,
            commands::prepare_game_java,
            commands::download_game_java,
            commands::cancel_java_download,
            commands::get_repositories,
            commands::select_repository,
            commands::add_new_repository,
            commands::get_account,
            commands::get_instances_in_repository,
            commands::select_instance,
            commands::get_version_manifest,
            commands::handle_clicked_on_version,
            core::version_export::export_minecraft_version,
            core::version_export::open_minecraft_wiki,
            core::installer::download_minecraft_version,
            commands::get_plugin_versions,
            commands::get_active_instance,
            util::server_query::server_query,
            util::skin::fetch_username_uuid,
            util::skin::fetch_uuid_profile,
            core::api_client::fetch_with_modrinth,
            util::skin::fetch_skin_from_uuid_cached,
            util::skin::fetch_skin_from_url,
            // scaffolding
            util::scaffolding::get_terracotta_meta,
            util::scaffolding::get_terracotta_state,
            util::scaffolding::set_terracotta_waiting,
            util::scaffolding::set_terracotta_host_scanning,
            util::scaffolding::set_terracotta_host_starting,
            util::scaffolding::set_terracotta_guesting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
