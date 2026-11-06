//! 系统级命令：外链、系统字体、今日人品和服务器列表 ping。

use pcl_core::link::mcping::{self, model::McPingResult};
use pcl_core::utils::{external, toys};

#[tauri::command]
pub async fn open_external_url(url: String) -> Result<(), String> {
    external::open_external_url(url).await
}

#[tauri::command]
pub async fn system_fonts() -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(|| {
        let mut db = fontdb::Database::new();
        db.load_system_fonts();
        let mut names: Vec<_> = db
            .faces()
            .flat_map(|face| face.families.iter().map(|(name, _)| name.clone()))
            .collect();
        names.sort();
        names.dedup();
        names
    })
    .await
    .map_err(|error| error.to_string())
}

#[tauri::command]
/// 获取今日人品，只考虑到日期
pub fn get_lucky_today() -> u8 {
    toys::get_lucky_today()
}

#[tauri::command]
pub async fn server_query(addr_str: &str) -> Result<(McPingResult, u128), String> {
    mcping::server_query(addr_str).await
}
