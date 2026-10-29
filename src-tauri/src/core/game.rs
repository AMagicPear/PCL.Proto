use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::core::{
    api_client::game::VersionDetails, java::JavaRuntime, launcher::GameLaunchError,
    mcmod::PluginType, repository::GameRepository,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum GameJava {
    Default,
    Custom(Arc<JavaRuntime>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameInstance {
    pub id: String,
    pub name: String,
    pub directory: PathBuf,
    pub jar_path: PathBuf,
    pub version: String,
    pub json_path: PathBuf,
    pub natives_path: PathBuf,
    pub game_java: GameJava,
    pub global_dir: PathBuf,
    pub plugin_type: PluginType,
}

#[derive(Debug)]
pub enum GameInstanceError {
    InvalidVersionFolder,
    InvalidVersionJson,
    InvalidVersionJar,
}

impl GameInstance {
    pub fn from_version_folder(
        version_folder: &Path,
        repo: &GameRepository,
    ) -> Result<Self, GameInstanceError> {
        let folder_name = version_folder
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or(GameInstanceError::InvalidVersionFolder)?;
        let json_path = version_folder.join(format!("{folder_name}.json"));
        if json_path.is_file() {
            let json_reader =
                fs::File::open(&json_path).map_err(|_| GameInstanceError::InvalidVersionJson)?;
            let json_content: serde_json::Value = serde_json::from_reader(json_reader)
                .map_err(|_| GameInstanceError::InvalidVersionJson)?;
            let id = json_content["id"]
                .as_str()
                .ok_or(GameInstanceError::InvalidVersionJson)?;
            let jar_name = if let Some(jar_name_in_json) = json_content["jar"].as_str() {
                jar_name_in_json // use jar name recorded in json file if there is
            } else {
                folder_name // use folder name as jar name if there is no jar name in json file
            };
            crate::util::file::validate_file_name(jar_name)
                .map_err(|_| GameInstanceError::InvalidVersionJar)?;
            let jar_path = version_folder.join(format!("{}.jar", jar_name));
            if !jar_path.exists() {
                log::error!("jar file not found in folder: {:?}", jar_path);
                return Err(GameInstanceError::InvalidVersionJar);
            }
            let version = {
                let patches = json_content["patches"].as_array();
                if let Some(patches) = patches {
                    let game_patch = patches.iter().find(|patch| patch["id"] == "game");
                    if let Some(game_patch) = game_patch {
                        game_patch["version"].as_str()
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            let version = version
                .or_else(|| json_content["clientVersion"].as_str())
                .unwrap_or(id)
                .to_string();
            Ok(GameInstance {
                id: id.to_string(),
                name: folder_name.to_string(),
                directory: version_folder.to_path_buf(),
                jar_path,
                version,
                json_path,
                natives_path: version_folder.join("natives"),
                game_java: GameJava::Default,
                global_dir: repo.path.clone(),
                plugin_type: PluginType::default(),
            })
        } else {
            log::error!("version json not found in folder: {:?}", version_folder);
            Err(GameInstanceError::InvalidVersionJson)
        }
    }

    pub fn read_version_json(&self) -> Result<VersionDetails, GameLaunchError> {
        let json_reader = std::fs::File::open(&self.json_path)?;
        let version_json: VersionDetails = serde_json::from_reader(json_reader)?;
        Ok(version_json)
    }
}

#[cfg(test)]
#[test]
fn from_version_folder() {
    let version_folder =
        PathBuf::from("/Users/amagicpear/HMCL/.minecraft/versions/Fabulously Optimized 1.21.5");
    let game_repo = GameRepository::new("HMCL", PathBuf::from("/Users/amagicpear/HMCL/.minecraft"));
    let game_repo = Arc::new(game_repo);
    let instance = GameInstance::from_version_folder(&version_folder, &game_repo);
    println!("{:?}", instance);
}
