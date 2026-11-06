//! CE Java management, with executable probing and discovery for each desktop OS.
//!
//! The module layout follows `PCL.Core.Minecraft.Java`:
//! `installation`, `entry`, `manager`, scanners, parser, installer and
//! user-preference types.

use std::time::{Duration, Instant};

pub mod entry;
pub mod install;
pub mod installation;
pub mod manager;
pub mod preference;
pub mod requirements;
mod scanner;
#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub use crate::utils::platform::{Architecture, Compatibility};
#[allow(unused_imports)]
pub use entry::{JavaEntry, JavaSource, JavaStorageItem};
#[allow(unused_imports)]
pub use installation::{JavaInstallation, JavaVersion, java_binary, launcher_directory};
pub use manager::JavaManager;
pub(crate) use scanner::scan;

static SCAN_LOCK: tokio::sync::Mutex<Option<Instant>> = tokio::sync::Mutex::const_new(None);

/// 已安装的 Java 与当前选择；由宿主应用持久化。
#[derive(Clone, Debug, Default)]
pub struct JavaState {
    pub runtimes: Vec<JavaEntry>,
    pub selected: Option<std::path::PathBuf>,
}

/// 扫描 Java 需要知道的目录：各 Minecraft 文件夹和启动器托管的运行时目录。
#[derive(Clone, Debug, Default)]
pub struct JavaRoots {
    pub repositories: Vec<std::path::PathBuf>,
    pub runtime_dir: std::path::PathBuf,
}

/// 重新扫描并合并 Java 列表；结果由宿主写回自己的配置。
pub async fn refresh(
    force: bool,
    state: &JavaState,
    roots: &JavaRoots,
) -> Result<JavaState, String> {
    let mut guard = SCAN_LOCK.lock().await;
    if !force && guard.is_some_and(|last| last.elapsed() < Duration::from_secs(13)) {
        return Ok(state.clone());
    }
    let mut directories = roots
        .repositories
        .iter()
        .map(|repository| repository.join("runtime"))
        .collect::<Vec<_>>();
    directories.push(roots.runtime_dir.clone());
    let known = state.runtimes.clone();
    let scanned = scan(known.clone(), directories).await?;
    let selected = state.selected.clone();
    let selected_probe = selected.clone();
    let actual_selected = tokio::task::spawn_blocking(move || {
        selected_probe
            .and_then(|path| JavaInstallation::probe(&path).ok())
            .map(|installation| installation.java_exe)
    })
    .await
    .map_err(|error| error.to_string())?;
    let merged = tokio::task::spawn_blocking(move || {
        let mut manager = JavaManager::from_entries(known.clone());
        manager.merge_scanned(scanned, &known);
        manager.sorted()
    })
    .await
    .map_err(|error| error.to_string())?;
    *guard = Some(Instant::now());
    Ok(JavaState {
        runtimes: merged,
        selected: if selected.is_some() {
            actual_selected
        } else {
            None
        },
    })
}

pub fn sort_list(list: &mut [JavaEntry]) {
    JavaManager::sort(list);
}
