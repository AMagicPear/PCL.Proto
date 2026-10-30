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
mod protocol;
mod setup;
mod util;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("pclinstance", |_app, request| protocol::serve(request))
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
            if let Ok(config_manager) = setup::CONFIG_MANAGER.as_ref() {
                app.manage(Arc::clone(&config_manager.app_state));
                app.manage(&config_manager.api_client);
                terracotta::init_lib(config_manager.identifier_path());
            } else {
                log::error!("Config manager failed: {:?}", setup::CONFIG_MANAGER.as_ref().err());
                app.dialog()
                    .message("Could not load the launcher configuration. The existing configuration has not been overwritten. Check the logs before restarting.")
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
            if setup::CONFIG_MANAGER.is_ok() {
                tauri::async_runtime::spawn(async move {
                    if let Err(error) = core::java::refresh(false).await {
                        log::error!("Java scan failed: {error}");
                    }
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::begin_launch,
            commands::cancel_launch,
            commands::launch_game,
            core::launch_session::get_game_sessions,
            core::launch_session::stop_game_session,
            core::launch_session::clear_game_session_log,
            core::downloader::resources::get_download_stats,
            core::downloader::resources::set_download_settings,
            core::downloader::resources::get_download_settings,
            core::downloader::resources::initialize_download_settings,
            setup::get_log_max_lines,
            setup::set_log_max_lines,
            setup::get_window_settings,
            setup::set_window_settings,
            setup::get_launcher_visibility,
            setup::set_launcher_visibility,
            commands::get_memory_settings,
            commands::set_memory_settings,
            commands::migrate_instance_memory_settings,
            commands::preview_memory_settings,
            commands::get_isolation_default,
            commands::set_isolation_default,
            commands::get_instance_isolation,
            commands::set_instance_isolation,
            commands::get_launch_arguments,
            commands::set_launch_arguments,
            commands::get_instance_launch_settings,
            commands::set_instance_launch_settings,
            commands::get_proxy_settings,
            commands::set_proxy_settings,
            commands::export_launch_script,
            commands::repair_instance_files,
            commands::get_instance_description,
            commands::set_instance_description,
            commands::rename_instance,
            commands::list_instance_resources,
            commands::open_instance_resource_folder,
            commands::open_instance_path,
            commands::set_instance_resources_enabled,
            commands::delete_instance_resources,
            commands::import_instance_resources,
            util::toys::get_lucky_today,
            commands::add_java,
            commands::get_java_list,
            commands::refresh_java_list,
            commands::get_java_settings,
            commands::set_global_java,
            commands::set_java_enabled,
            commands::set_instance_java_settings,
            commands::migrate_instance_java_settings,
            commands::resolve_legacy_instance_ids,
            commands::select_relative_java,
            commands::open_java_folder,
            commands::open_instance_folder,
            commands::prepare_game_java,
            commands::download_game_java,
            commands::cancel_java_download,
            commands::get_repositories,
            commands::select_repository,
            commands::add_new_repository,
            commands::get_account,
            commands::get_instances_in_repository,
            commands::instance_folder_action,
            commands::select_instance,
            commands::get_version_manifest,
            commands::handle_clicked_on_version,
            core::version_export::export_minecraft_version,
            core::version_export::open_minecraft_wiki,
            core::loader_export::save_loader_file,
            core::loader_export::open_loader_changelog,
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
