use std::{fs, path::PathBuf};

use crate::core::game::GameInstance;

/// a .minecraft folder, not a single version folder
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct GameRepository {
    pub name: String,
    pub path: PathBuf,
}

impl GameRepository {
    pub fn new(name: &str, path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            path,
        }
    }
    /// Scan on request: installs and external launchers can change this directory.
    pub fn game_instances(&self) -> Vec<GameInstance> {
        let versions_folder = self.path.join("versions");
        let entries = match fs::read_dir(&versions_folder) {
            Ok(entries) => entries,
            Err(error) => {
                log::debug!("Cannot scan {}: {error}", versions_folder.display());
                return Vec::new();
            }
        };
        let mut instances = Vec::new();
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    log::warn!("Cannot read instance entry: {error}");
                    continue;
                }
            };
            if !entry.file_type().is_ok_and(|kind| kind.is_dir()) {
                continue;
            }
            match GameInstance::from_version_folder(&entry.path(), self) {
                Ok(instance) => instances.push(instance),
                Err(error) => log::debug!(
                    "Skipping incomplete or invalid instance {}: {error:?}",
                    entry.path().display()
                ),
            }
        }
        instances.sort_by(|a, b| a.name.cmp(&b.name));
        instances
    }
}
