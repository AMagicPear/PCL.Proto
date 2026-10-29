//! Mod for launching a Minecraft game
//! Code referenced from xphost/MMCLL/src/launcher.rs
//! MIT License | https://github.com/xphost008/MMCLL
//! Code referenced from scl-core/src/client.rs
//! LGPL-3.0 License & scl License | https://github.com/Steve-xmh/scl
//! Start script referenced from HMCL
//! GPL-3.0 License | https://github.com/HMCL-dev/HMCL
// use crate::setup::constants::LAUNCHER_NAME;

use crate::core::api_client::game::VersionDetails;
use crate::setup::constants::{APP_VERSION, LAUNCHER_NAME};
use crate::{
    core::{auth::Account, game::GameInstance, java::JavaRuntime},
    setup::AppState,
};
use std::sync::Arc;

/// Essential options for launching a Minecraft game
pub struct LaunchOption {
    account: Arc<Account>,
    java_runtime: Arc<JavaRuntime>,
    game_instance: Arc<GameInstance>,
    max_memory: usize,
    width: Option<usize>,
    height: Option<usize>,
    version_details: VersionDetails,
}

#[derive(Debug, thiserror::Error)]
pub enum GameLaunchError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Json parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid Minecraft metadata: {0}")]
    InvalidMetadata(String),

    #[error("active account not found")]
    MissingAccount,

    #[error("active game instance not found")]
    MissingGameInstance,
}

impl LaunchOption {
    pub fn with_window_size(mut self, width: usize, height: usize) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn launch_checked(&self) -> Result<std::process::Child, GameLaunchError> {
        // check account
        // check all the libraries
        self.launch()
    }

    /// Launch a Minecraft game with the given options
    fn launch(&self) -> Result<std::process::Child, GameLaunchError> {
        let mut command = std::process::Command::new(&self.java_runtime.java_exe);
        command
            .args(self.build_jvm_arguments()) // build jvm arguments
            .arg("-cp")
            .arg(build_classpath(&self.game_instance, &self.version_details)?)
            .arg(&self.version_details.main_class)
            .args(self.build_game_arguments())
            .current_dir(&self.game_instance.global_dir);
        Ok(command.spawn()?)
    }

    fn build_jvm_arguments(&self) -> Vec<String> {
        let mut args = Vec::new();
        // memory setting
        args.push(format!("-Xmx{}m", self.max_memory));
        // encoding settings
        args.push("-Dfile.encoding=UTF-8".to_string());
        args.push("-Dstdout.encoding=UTF-8".to_string());
        args.push("-Dstderr.encoding=UTF-8".to_string());
        // safety settings
        args.push("-Djava.rmi.server.useCodebaseOnly=true".to_string());
        args.push("-Dcom.sun.jndi.rmi.object.trustURLCodebase=false".to_string());
        args.push("-Dcom.sun.jndi.cosnaming.object.trustURLCodebase=false".to_string());
        args.push("-Dlog4j2.formatMsgNoLookups=true".to_string());
        if let Some(logging) = &self.version_details.logging {
            let path = self
                .game_instance
                .global_dir
                .join("assets/log_configs")
                .join(&logging.client.file.id);
            args.push(
                logging
                    .client
                    .argument
                    .replace("${path}", &path.to_string_lossy()),
            );
        }
        // jar file
        args.push(format!(
            "-Dminecraft.client.jar={}",
            self.game_instance.jar_path.display()
        ));
        // macOS specific settings
        #[cfg(target_os = "macos")]
        {
            args.push("-XstartOnFirstThread".to_string());
            args.push("-Xdock:name=Minecraft".to_string());
            // TODO: 图标路径需要从 assets 中获取
        }
        // native libraries path
        let natives_path = &self.game_instance.natives_path.display();
        args.push(format!("-Djava.library.path={}", natives_path));
        args.push(format!("-Djna.tmpdir={}", natives_path));
        args.push(format!(
            "-Dorg.lwjgl.system.SharedLibraryExtractPath={}",
            natives_path
        ));
        args.push(format!("-Dio.netty.native.workdir={}", natives_path));
        // launcher info
        args.push(format!("-Dminecraft.launcher.brand={}", LAUNCHER_NAME));
        args.push(format!("-Dminecraft.launcher.version={}", APP_VERSION));
        // gc optimize
        args.append(
            &mut [
                "-XX:+UnlockExperimentalVMOptions",
                "-XX:+UnlockDiagnosticVMOptions",
                "-XX:+UseG1GC",
                "-XX:G1MixedGCCountTarget=5",
                "-XX:G1NewSizePercent=20",
                "-XX:G1ReservePercent=20",
                "-XX:MaxGCPauseMillis=50",
                "-XX:G1HeapRegionSize=32m",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );
        args
    }

    fn build_game_arguments(&self) -> Vec<String> {
        vec![
            format!("--username={}", (self.account.username())),
            format!("--version={}", self.game_instance.version),
            format!("--gameDir={}", self.game_instance.directory.display()),
            format!(
                "--assetsDir={}/assets",
                self.game_instance.global_dir.display()
            ),
            format!("--assetIndex={}", &self.version_details.assets),
            format!("--uuid={}", self.account.uuid()),
            format!(
                "--accessToken={}",
                self.account.access_token().unwrap_or("0")
            ),
            format!("--userType={}", "msa"),
            format!("--versionType={}", LAUNCHER_NAME),
            format!("--width={}", self.width.unwrap_or(854)),
            format!("--height={}", self.height.unwrap_or(480)),
        ]
    }

    /// build a launch option from app state if it is possible
    pub fn from_state(
        state: &AppState,
        java_selected: Arc<JavaRuntime>,
    ) -> Result<Self, GameLaunchError> {
        if let Some(game_instance) = state.active_game_instance.as_ref() {
            let active_account = state.active_account.as_ref();
            if active_account.is_none() {
                return Err(GameLaunchError::MissingAccount);
            }
            return Ok(Self {
                account: active_account.unwrap().clone(),
                java_runtime: java_selected.clone(),
                game_instance: game_instance.clone(),
                max_memory: state.pcl_setup_info.max_memory,
                width: None,
                height: None,
                version_details: game_instance.read_version_json()?,
            });
        }
        Err(GameLaunchError::MissingGameInstance)
    }
}

fn build_classpath(
    instance: &GameInstance,
    details: &VersionDetails,
) -> Result<String, GameLaunchError> {
    let mut classpath = Vec::new();
    let context = super::minecraft::RuleContext::current();
    for lib in &details.libraries {
        if !lib
            .rule_allow(&context)
            .map_err(|e| GameLaunchError::InvalidMetadata(e.to_string()))?
        {
            continue;
        }
        let Some(artifact) = &lib.downloads.artifact else {
            continue;
        };
        let path = super::minecraft::library_path(&instance.global_dir, artifact)
            .map_err(|e| GameLaunchError::InvalidMetadata(e.to_string()))?;
        if !path.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Missing library: {}", path.display()),
            )
            .into());
        }
        classpath.push(path);
    }
    if !instance.jar_path.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Minecraft client JAR not found",
        )
        .into());
    }
    classpath.push(instance.jar_path.clone());
    let joined = std::env::join_paths(classpath)
        .map_err(|e| GameLaunchError::InvalidMetadata(e.to_string()))?;
    joined
        .into_string()
        .map_err(|_| GameLaunchError::InvalidMetadata("Classpath is not valid Unicode".into()))
}

#[cfg(test)]
#[test]
pub fn game_launch_test() {
    use crate::core::repository::GameRepository;
    use std::path::PathBuf;
    use std::sync::Arc;

    let account = Arc::new(Account::Offline {
        username: "AMagicPear".to_string(),
        uuid: "12345678-1234-1234-1234-123456789012".to_string(),
    });

    let game_repo = GameRepository::new("HMCL", PathBuf::from("/Users/amagicpear/HMCL/.minecraft"));
    let game_repo = Arc::new(game_repo);
    let version_folder = PathBuf::from("/Users/amagicpear/HMCL/.minecraft/versions/1.21.8");
    let game_instance =
        Arc::new(GameInstance::from_version_folder(&version_folder, &game_repo).unwrap());
    let launch_option = LaunchOption {
        account,
        java_runtime: Arc::new(JavaRuntime::try_from("/usr/bin/java").unwrap()),
        game_instance: game_instance.clone(),
        max_memory: 4096,
        width: None,
        height: None,
        version_details: game_instance.read_version_json().unwrap(),
    }
    .with_window_size(1280, 720);
    if let Ok(mut child) = launch_option.launch() {
        child.wait().unwrap();
    } else {
        eprintln!(
            "launch failed, instance id:{:?}",
            launch_option.game_instance.id
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;

    #[test]
    fn classpath_uses_install_rules_and_reports_missing_libraries() {
        let root =
            std::env::temp_dir().join(format!("pcl-classpath-test-{}", uuid::Uuid::new_v4()));
        let version = root.join("versions/Custom");
        fs::create_dir_all(&version).unwrap();
        fs::create_dir_all(root.join("libraries/test")).unwrap();
        fs::write(root.join("libraries/test/allowed.jar"), b"library").unwrap();
        fs::write(version.join("Custom.jar"), b"client").unwrap();
        let artifact = |path| json!({"sha1":"a".repeat(40), "size":1,"url":"https://example.org/library.jar","path":path});
        let metadata = json!({
            "id":"Custom", "clientVersion":"1.test", "type":"release", "assets":"test", "mainClass":"Main",
            "downloads":{"client":artifact("client.jar")},
            "assetIndex":{"id":"test","sha1":"a".repeat(40),"size":1,"url":"https://example.org/index.json"},
            "libraries":[
                {"name":"test:allowed:1","downloads":{"artifact":artifact("test/allowed.jar")}},
                {"name":"test:blocked:1","rules":[{"action":"disallow"}],"downloads":{"artifact":artifact("test/absent.jar")}},
                {"name":"test:native:1","downloads":{"classifiers":{}}}
            ]
        });
        fs::write(
            version.join("Custom.json"),
            serde_json::to_vec(&metadata).unwrap(),
        )
        .unwrap();
        let repo = super::super::repository::GameRepository::new("test", root.clone());
        let instance = GameInstance::from_version_folder(&version, &repo).unwrap();
        let details = instance.read_version_json().unwrap();
        let classpath = build_classpath(&instance, &details).unwrap();
        assert_eq!(
            std::env::split_paths(&classpath).collect::<Vec<_>>(),
            vec![
                root.join("libraries/test/allowed.jar"),
                version.join("Custom.jar")
            ]
        );
        fs::remove_file(root.join("libraries/test/allowed.jar")).unwrap();
        assert!(
            build_classpath(&instance, &details)
                .unwrap_err()
                .to_string()
                .contains("Missing library")
        );
        fs::remove_dir_all(root).unwrap();
    }
}
