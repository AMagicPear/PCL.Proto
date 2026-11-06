//! 实例隔离的判定与落盘。判定规则在 `pcl-core`，这里负责读写配置。

use std::path::{Path, PathBuf};

use pcl_core::minecraft::instance::McInstance;
use pcl_core::minecraft::instance::isolation::{self, IsolationDefault};
use pcl_core::minecraft::java::preference::current_instance;

use crate::config::states::AppState;

pub fn ensure(state: &mut AppState, path: &Path) -> Result<bool, String> {
    let instance = current_instance(state.active_game_instance.as_deref(), path)
        .map_err(|error| error.to_string())?;
    if let Some(value) = state.instance_isolation.get(path) {
        return Ok(*value);
    }
    let instance = instance
        .refresh()
        .map_err(|error| format!("Cannot refresh Minecraft instance: {error:?}"))?;
    let value = isolation::initial(&instance, state.isolation_default)
        .map_err(|error| format!("{error:#}"))?;
    state.instance_isolation.insert(PathBuf::from(path), value);
    Ok(value)
}

pub fn set_default(state: &mut AppState, value: IsolationDefault) -> Result<(), String> {
    if state.isolation_default == value {
        return Ok(());
    }
    for repo in &state.repositories {
        if !repo.path.is_dir() {
            return Err(format!(
                "Minecraft folder is unavailable: {}",
                repo.path.display()
            ));
        }
        let versions = repo.path.join("versions");
        let entries = match std::fs::read_dir(&versions) {
            Ok(entries) => entries,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => return Err(format!("Cannot inspect {}: {error}", versions.display())),
        };
        for entry in entries {
            let entry = entry.map_err(|error| error.to_string())?;
            if !entry
                .file_type()
                .map_err(|error| error.to_string())?
                .is_dir()
            {
                continue;
            }
            let path = entry.path();
            if state.instance_isolation.contains_key(&path) {
                continue;
            }
            if let Ok(instance) = McInstance::from_version_folder(&path, repo) {
                let decision =
                    isolation::initial(&instance, state.isolation_default).map_err(|error| {
                        format!("Cannot decide isolation for {}: {error:#}", path.display())
                    })?;
                state.instance_isolation.insert(path, decision);
            }
        }
    }
    state.isolation_default = value;
    Ok(())
}
