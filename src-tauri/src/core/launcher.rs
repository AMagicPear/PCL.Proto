//! Mod for launching a Minecraft game
//! Code referenced from xphost/MMCLL/src/launcher.rs
//! MIT License | https://github.com/xphost008/MMCLL
//! Code referenced from scl-core/src/client.rs
//! LGPL-3.0 License & scl License | https://github.com/Steve-xmh/scl
//! Start script referenced from HMCL
//! GPL-3.0 License | https://github.com/HMCL-dev/HMCL
// use crate::setup::constants::LAUNCHER_NAME;

use crate::core::api_client::game::VersionDetails;
use crate::core::minecraft::{Argument, ArgumentValue, RuleContext, rules_allow};
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
    game_directory: std::path::PathBuf,
    max_memory: usize,
    initial_memory: Option<usize>,
    width: Option<usize>,
    height: Option<usize>,
    launcher_size: bool,
    version_details: VersionDetails,
    natives_path: std::path::PathBuf,
    arguments: crate::core::launch_arguments::LaunchArguments,
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

    #[error("Memory allocation failed: {0}")]
    Memory(String),

    #[error("Invalid custom launch arguments: {0}")]
    CustomArguments(String),
}

impl LaunchOption {
    pub fn with_window_size(mut self, width: usize, height: usize) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn uses_launcher_size(&self) -> bool {
        self.launcher_size
    }

    pub fn launch_checked(&self) -> Result<std::process::Child, GameLaunchError> {
        // check account
        // check all the libraries
        self.launch()
    }

    /// Launch a Minecraft game with the given options
    fn launch(&self) -> Result<std::process::Child, GameLaunchError> {
        let mut command = std::process::Command::new(&self.java_runtime.java_exe);
        let context = RuleContext::for_java(&self.java_runtime.architecture)
            .map_err(|e| GameLaunchError::InvalidMetadata(e.to_string()))?;
        let head = self
            .arguments
            .classpath_paths()
            .map_err(GameLaunchError::CustomArguments)?;
        let classpath =
            build_classpath(&self.game_instance, &self.version_details, &context, &head)?;
        let (custom_jvm, mut jvm, game) = self.metadata_arguments(&classpath)?;
        let custom_initial = custom_jvm.iter().any(|arg| arg.starts_with("-Xms"));
        jvm.retain(|arg| {
            !arg.starts_with("-Xmx")
                && !(arg.starts_with("-Xms") && (custom_initial || self.initial_memory.is_some()))
        });
        command
            .args(custom_jvm)
            .args(self.build_jvm_arguments(custom_initial))
            .args(jvm)
            .arg(&self.version_details.main_class)
            .args(game)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .current_dir(&self.game_directory);
        Ok(command.spawn()?)
    }

    pub fn access_token(&self) -> Option<String> {
        self.account.access_token().map(str::to_owned)
    }

    fn build_jvm_arguments(&self, custom_initial: bool) -> Vec<String> {
        let mut args = Vec::new();
        // memory setting
        args.push(format!("-Xmx{}m", self.max_memory));
        if let Some(initial) = self.initial_memory.filter(|_| !custom_initial) {
            args.push(format!("-Xms{initial}m"));
        }
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
        let natives_path = &self.natives_path.display();
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
        args
    }

    fn metadata_arguments(
        &self,
        classpath: &str,
    ) -> Result<(Vec<String>, Vec<String>, Vec<String>), GameLaunchError> {
        let instance = &self.game_instance;
        let assets = instance.global_dir.join("assets");
        let xuid = match self.account.as_ref() {
            Account::Microsoft { xuid, .. } => xuid.as_str(),
            _ => "0",
        };
        let vars = std::collections::HashMap::from([
            ("classpath", classpath.to_owned()),
            (
                "classpath_separator",
                if cfg!(windows) { ";" } else { ":" }.into(),
            ),
            (
                "natives_directory",
                self.natives_path.to_string_lossy().into_owned(),
            ),
            (
                "library_directory",
                instance
                    .global_dir
                    .join("libraries")
                    .to_string_lossy()
                    .into_owned(),
            ),
            (
                "libraries_directory",
                instance
                    .global_dir
                    .join("libraries")
                    .to_string_lossy()
                    .into_owned(),
            ),
            ("launcher_name", LAUNCHER_NAME.into()),
            ("launcher_version", APP_VERSION.into()),
            ("version_name", instance.name.clone()),
            ("version_type", LAUNCHER_NAME.into()),
            (
                "game_directory",
                self.game_directory.to_string_lossy().into_owned(),
            ),
            ("assets_root", assets.to_string_lossy().into_owned()),
            (
                "game_assets",
                assets.join("virtual/legacy").to_string_lossy().into_owned(),
            ),
            (
                "assets_index_name",
                self.version_details.asset_index.id.clone(),
            ),
            ("auth_player_name", self.account.username().into()),
            ("auth_uuid", self.account.uuid().into()),
            (
                "auth_access_token",
                self.account.access_token().unwrap_or("0").into(),
            ),
            (
                "access_token",
                self.account.access_token().unwrap_or("0").into(),
            ),
            (
                "auth_session",
                self.account.access_token().unwrap_or("0").into(),
            ),
            ("auth_xuid", xuid.into()),
            ("clientid", String::new()),
            ("user_type", "msa".into()),
            ("user_properties", "{}".into()),
            ("resolution_width", self.width.unwrap_or(854).to_string()),
            ("resolution_height", self.height.unwrap_or(480).to_string()),
        ]);
        let mut context = RuleContext::for_java(&self.java_runtime.architecture)
            .map_err(|e| GameLaunchError::InvalidMetadata(e.to_string()))?;
        context
            .features
            .insert("has_custom_resolution".into(), true);
        let expand = |input: &str| -> Result<String, GameLaunchError> {
            let pattern = regex::Regex::new(r"\$\{([^}]+)\}").unwrap();
            let mut missing = None;
            let expanded = pattern.replace_all(input, |captures: &regex::Captures<'_>| {
                vars.get(&captures[1]).cloned().unwrap_or_else(|| {
                    missing = Some(captures[1].to_owned());
                    String::new()
                })
            });
            if let Some(name) = missing {
                return Err(GameLaunchError::InvalidMetadata(format!(
                    "Unknown argument placeholder: {name}"
                )));
            }
            Ok(expanded.into_owned())
        };
        let parse = |arguments: &[Argument]| -> Result<Vec<String>, GameLaunchError> {
            let mut result = Vec::new();
            for argument in arguments {
                match argument {
                    Argument::Plain(value) => result.push(expand(value)?),
                    Argument::Conditional { rules, value }
                        if rules_allow(Some(rules), &context)
                            .map_err(|e| GameLaunchError::InvalidMetadata(e.to_string()))? =>
                    {
                        match value {
                            ArgumentValue::One(value) => result.push(expand(value)?),
                            ArgumentValue::Many(values) => {
                                for value in values {
                                    result.push(expand(value)?);
                                }
                            }
                        }
                    }
                    Argument::Conditional { .. } => {}
                }
            }
            Ok(result)
        };
        let metadata = self.version_details.arguments.as_ref();
        let mut jvm = match metadata.filter(|a| !a.jvm.is_empty()) {
            Some(arguments) => parse(&arguments.jvm)?,
            None => Vec::new(),
        };
        if !jvm.iter().any(|arg| {
            matches!(arg.as_str(), "-cp" | "-classpath" | "--class-path")
                || arg.starts_with("--class-path=")
        }) {
            jvm.extend(["-cp".into(), classpath.into()]);
        }
        let mut game = if let Some(arguments) = metadata.filter(|a| !a.game.is_empty()) {
            parse(&arguments.game)?
        } else if let Some(legacy) = self.version_details.minecraft_arguments.as_deref() {
            let mut game = shlex::split(legacy)
                .ok_or_else(|| {
                    GameLaunchError::InvalidMetadata("Invalid legacy game arguments".into())
                })?
                .iter()
                .map(|argument| expand(argument))
                .collect::<Result<Vec<_>, _>>()?;
            if self
                .version_details
                .release_time
                .as_deref()
                .is_some_and(|date| date >= "2013-06-17")
                && !game
                    .iter()
                    .any(|arg| arg == "--height" || arg.starts_with("--height="))
            {
                game.extend([
                    "--height".into(),
                    self.height.unwrap_or(480).to_string(),
                    "--width".into(),
                    self.width.unwrap_or(854).to_string(),
                ]);
            }
            game
        } else {
            return Err(GameLaunchError::InvalidMetadata(
                "Missing Minecraft game arguments".into(),
            ));
        };
        let parse_custom = |value: &str| -> Result<Vec<String>, GameLaunchError> {
            shlex::split(value)
                .ok_or_else(|| GameLaunchError::CustomArguments("Unclosed quote or escape".into()))?
                .iter()
                .map(|argument| expand(argument))
                .collect()
        };
        let mut custom_jvm = parse_custom(&self.arguments.jvm)?;
        let last_initial = custom_jvm.iter().rposition(|arg| arg.starts_with("-Xms"));
        custom_jvm = custom_jvm
            .into_iter()
            .enumerate()
            .filter_map(|(index, arg)| {
                if arg.starts_with("-Xmx")
                    || (arg.starts_with("-Xms") && Some(index) != last_initial)
                {
                    None
                } else {
                    Some(arg)
                }
            })
            .collect();
        game.extend(parse_custom(&self.arguments.game)?);
        Ok((custom_jvm, jvm, game))
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
            let game_instance = Arc::new(game_instance.refresh().map_err(|error| {
                GameLaunchError::InvalidMetadata(format!(
                    "Cannot refresh Minecraft instance: {error:?}"
                ))
            })?);
            let window = state.window_settings.as_ref();
            let game_directory =
                crate::core::isolation::directory(state, &game_instance).to_owned();
            let memory = state
                .instance_memory_settings
                .get(&game_instance.directory)
                .or(state.memory_settings.as_ref());
            let (max_memory, initial_memory) = if let Some(settings) = memory {
                let available = if matches!(settings, crate::core::memory::MemorySettings::Auto) {
                    let mut system = sysinfo::System::new();
                    system.refresh_memory();
                    system.available_memory()
                } else {
                    0
                };
                crate::core::memory::resolve(
                    settings,
                    available,
                    &game_instance,
                    &game_directory,
                    java_selected.is_64_bit,
                )
                .map_err(GameLaunchError::Memory)?
            } else {
                (state.pcl_setup_info.max_memory, None)
            };
            let custom =
                window.filter(|settings| settings.mode == crate::setup::WindowMode::Custom);
            let global = state
                .launch_arguments
                .clone()
                .unwrap_or_else(crate::core::launch_arguments::LaunchArguments::global_default);
            let arguments = state
                .instance_launch_arguments
                .get(&game_instance.directory)
                .map(|instance| instance.effective(&global))
                .unwrap_or(global);
            arguments
                .validate(true)
                .map_err(GameLaunchError::CustomArguments)?;
            return Ok(Self {
                account: active_account.unwrap().clone(),
                java_runtime: java_selected.clone(),
                game_instance: game_instance.clone(),
                game_directory,
                max_memory,
                initial_memory,
                width: custom.map(|settings| settings.width as usize),
                height: custom.map(|settings| settings.height as usize),
                launcher_size: window
                    .is_some_and(|settings| settings.mode == crate::setup::WindowMode::Launcher),
                version_details: game_instance.read_version_json()?,
                natives_path: game_instance.natives_path.clone(),
                arguments,
            });
        }
        Err(GameLaunchError::MissingGameInstance)
    }

    pub async fn prepare_natives(
        &mut self,
        downloader: &super::downloader::Downloader,
    ) -> anyhow::Result<super::installer::LaunchNatives> {
        let context = RuleContext::for_java(&self.java_runtime.architecture)?;
        let natives = super::installer::prepare_launch_libraries(
            &self.version_details,
            &self.game_instance.global_dir,
            &self.game_instance.directory,
            &context,
            downloader,
        )
        .await?;
        self.natives_path = natives.path().to_owned();
        Ok(natives)
    }

    pub async fn prepare_legacy_assets(
        &self,
        downloader: &super::downloader::Downloader,
    ) -> anyhow::Result<()> {
        super::installer::prepare_legacy_assets(
            &self.version_details,
            &self.game_instance.global_dir,
            &self.game_directory,
            downloader,
        )
        .await
    }
}

fn build_classpath(
    instance: &GameInstance,
    details: &VersionDetails,
    context: &RuleContext,
    head: &[std::path::PathBuf],
) -> Result<String, GameLaunchError> {
    let mut classpath: Vec<_> = head.iter().rev().cloned().collect();
    let mut seen = std::collections::HashSet::new();
    for lib in &details.libraries {
        if !lib
            .rule_allow(context)
            .map_err(|e| GameLaunchError::InvalidMetadata(e.to_string()))?
        {
            continue;
        }
        let Some(path) = lib
            .classpath_path(&instance.global_dir, &instance.directory)
            .map_err(|e| GameLaunchError::InvalidMetadata(e.to_string()))?
        else {
            continue;
        };
        let key = lib
            .classpath_key()
            .map_err(|e| GameLaunchError::InvalidMetadata(e.to_string()))?;
        if !seen.insert(key) {
            continue;
        }
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
#[ignore = "Requires a specific local Minecraft installation and starts a real game"]
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
        game_directory: game_instance.directory.clone(),
        max_memory: 4096,
        initial_memory: None,
        width: None,
        height: None,
        launcher_size: false,
        version_details: game_instance.read_version_json().unwrap(),
        natives_path: game_instance.natives_path.clone(),
        arguments: crate::core::launch_arguments::LaunchArguments::global_default(),
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

    #[cfg(unix)]
    #[test]
    fn metadata_arguments_reach_the_process_without_shell_splitting() {
        use std::os::unix::fs::PermissionsExt;
        let root = std::env::temp_dir().join(format!("pcl-launch-test-{}", uuid::Uuid::new_v4()));
        let version = root.join("versions/Custom Name");
        fs::create_dir_all(&version).unwrap();
        fs::write(version.join("Custom Name.jar"), b"client").unwrap();
        let artifact =
            json!({"sha1":"a".repeat(40), "size":1, "url":"https://example.org/client.jar"});
        let metadata = json!({
            "id":"Custom Name", "type":"release", "assets":"test", "mainClass":"Main",
            "downloads":{"client":artifact}, "assetIndex":{"id":"test","sha1":"a".repeat(40),"size":1,"url":"https://example.org/index"}, "libraries":[],
            "arguments":{
                "jvm":["-cp","${classpath}", "-Dnative.fixture=${natives_directory}", {"rules":[{"action":"allow","os":{"arch":"aarch64"}}],"value":"-Darm=true"}, {"rules":[{"action":"allow","features":{"has_custom_resolution":true}}],"value":["-Dwidth=${resolution_width}","-Dlauncher=${launcher_name}"]}, {"rules":[{"action":"allow","features":{"is_demo_user":true}}],"value":"-Ddemo"}],
                "game":["--username","${auth_player_name}","--gameDir","${game_directory}","--accessToken","${auth_access_token}"]
            }
        });
        fs::write(
            version.join("Custom Name.json"),
            serde_json::to_vec(&metadata).unwrap(),
        )
        .unwrap();
        let executable = root.join("fake-java");
        fs::write(
            &executable,
            "#!/bin/sh\noutput=\"$(dirname \"$0\")/captured\"\npwd > \"$output\"\nprintf '%s\\n' \"$@\" >> \"$output\"\n",
        )
        .unwrap();
        fs::set_permissions(&executable, fs::Permissions::from_mode(0o755)).unwrap();
        let repo = super::super::repository::GameRepository::new("test", root.clone());
        let instance = Arc::new(GameInstance::from_version_folder(&version, &repo).unwrap());
        let runtime: JavaRuntime = serde_json::from_value(json!({
            "directory_path":root, "is_user_imported":false, "version":"21", "slug_version":21,
            "is_64_bit":true, "architecture":"Arm64", "compability":"Perfect", "is_jdk":true,
            "java_exe":executable, "implementor":"fixture"
        }))
        .unwrap();
        let mut option = LaunchOption {
            account: Arc::new(Account::Yggdrasil {
                username: "Name With Space".into(),
                uuid: "uuid".into(),
                access_token: "secret token".into(),
            }),
            java_runtime: Arc::new(runtime),
            game_instance: instance.clone(),
            game_directory: instance.directory.clone(),
            max_memory: 2048,
            initial_memory: None,
            width: Some(1024),
            height: Some(768),
            launcher_size: false,
            version_details: instance.read_version_json().unwrap(),
            natives_path: instance.natives_path.clone(),
            arguments: crate::core::launch_arguments::LaunchArguments::global_default(),
        };
        assert!(option.launch_checked().unwrap().wait().unwrap().success());
        let output = fs::read_to_string(root.join("captured")).unwrap();
        let lines: Vec<_> = output.lines().collect();
        assert_eq!(
            fs::canonicalize(lines[0]).unwrap(),
            fs::canonicalize(&version).unwrap()
        );
        assert!(lines.contains(&"-Dwidth=1024"));
        assert!(lines.contains(&"-Darm=true"));
        assert!(
            lines.contains(
                &format!("-Dnative.fixture={}", instance.natives_path.display()).as_str()
            )
        );
        assert!(!lines.contains(&"-Ddemo"));
        assert!(
            lines
                .windows(2)
                .any(|pair| pair == ["--username", "Name With Space"])
        );
        assert!(
            lines
                .windows(2)
                .any(|pair| pair == ["--gameDir", version.to_str().unwrap()])
        );
        assert!(
            lines
                .windows(2)
                .any(|pair| pair == ["--accessToken", "secret token"])
        );
        let state = AppState {
            active_account: Some(option.account.clone()),
            active_game_instance: Some(instance.clone()),
            instance_isolation: [(version.clone(), false)].into(),
            ..Default::default()
        };
        let shared = LaunchOption::from_state(&state, option.java_runtime.clone()).unwrap();
        assert!(shared.launch_checked().unwrap().wait().unwrap().success());
        let shared_output = fs::read_to_string(root.join("captured")).unwrap();
        let shared_lines: Vec<_> = shared_output.lines().collect();
        assert_eq!(
            fs::canonicalize(shared_lines[0]).unwrap(),
            fs::canonicalize(&root).unwrap()
        );
        assert!(
            shared_lines
                .windows(2)
                .any(|pair| pair == ["--gameDir", root.to_str().unwrap()])
        );
        let mut custom_state = state.clone();
        custom_state.memory_settings = Some(crate::core::memory::MemorySettings::Custom {
            maximum_mib: 3072,
            initial_mib: Some(1024),
        });
        custom_state.launch_arguments = Some(crate::core::launch_arguments::LaunchArguments {
            jvm: "-Dglobal=true".into(),
            game: "--server 'my host' --asset ${assets_root}".into(),
            classpath: String::new(),
        });
        let first_head = root.join("first extra.jar");
        let second_head = root.join("second.jar");
        custom_state.instance_launch_arguments.insert(version.clone(), crate::core::launch_arguments::LaunchArguments {
            jvm: "-XX:+UseZGC -Xmx8G -Xms256m -Xms512m -Dfixture='two words' -Droot=${game_directory}".into(),
            game: String::new(), classpath: format!("{};{}", first_head.display(), second_head.display()),
        });
        let custom = LaunchOption::from_state(&custom_state, option.java_runtime.clone()).unwrap();
        assert!(custom.launch_checked().unwrap().wait().unwrap().success());
        let output = fs::read_to_string(root.join("captured")).unwrap();
        let lines: Vec<_> = output.lines().collect();
        assert!(lines.contains(&"-XX:+UseZGC"));
        assert!(!lines.contains(&"-XX:+UseG1GC"));
        assert!(!lines.contains(&"-Dglobal=true"));
        assert!(lines.contains(&"-Dfixture=two words"));
        assert!(lines.contains(&format!("-Droot={}", root.display()).as_str()));
        assert_eq!(
            lines
                .iter()
                .filter(|arg| arg.starts_with("-Xmx"))
                .copied()
                .collect::<Vec<_>>(),
            ["-Xmx3072m"]
        );
        assert_eq!(
            lines
                .iter()
                .filter(|arg| arg.starts_with("-Xms"))
                .copied()
                .collect::<Vec<_>>(),
            ["-Xms512m"]
        );
        assert!(lines.windows(2).any(|pair| pair == ["--server", "my host"]));
        assert!(
            lines
                .windows(2)
                .any(|pair| pair == ["--asset", root.join("assets").to_str().unwrap()])
        );
        let cp = lines.windows(2).find(|pair| pair[0] == "-cp").unwrap()[1];
        assert_eq!(
            std::env::split_paths(cp).take(2).collect::<Vec<_>>(),
            [second_head, first_head]
        );
        option.version_details.arguments = Some(
            serde_json::from_value(
                json!({"jvm":["-Dloader=true"],"game":["--username","${auth_player_name}"]}),
            )
            .unwrap(),
        );
        assert_eq!(
            option.metadata_arguments("a path").unwrap().1,
            ["-Dloader=true", "-cp", "a path"]
        );
        option.version_details.arguments = None;
        option.version_details.minecraft_arguments = Some(
            "--username ${auth_player_name} --title 'Legacy Game' --assetsDir ${assets_root}"
                .into(),
        );
        let (_, jvm, game) = option.metadata_arguments("a path").unwrap();
        assert_eq!(jvm, ["-cp", "a path"]);
        assert_eq!(game[1], "Name With Space");
        assert_eq!(game[3], "Legacy Game");
        option.version_details.release_time = Some("2013-06-17T00:00:00Z".into());
        let (_, _, game) = option.metadata_arguments("a path").unwrap();
        assert!(game.windows(2).any(|pair| pair == ["--width", "1024"]));
        assert!(game.windows(2).any(|pair| pair == ["--height", "768"]));
        assert!(option.launch_checked().unwrap().wait().unwrap().success());
        let captured = fs::read_to_string(root.join("captured")).unwrap();
        let lines: Vec<_> = captured.lines().collect();
        assert!(lines.windows(2).any(|pair| pair == ["--width", "1024"]));
        assert!(lines.windows(2).any(|pair| pair == ["--height", "768"]));
        option.version_details.release_time = Some("2013-06-16T00:00:00Z".into());
        assert!(
            !option
                .metadata_arguments("a path")
                .unwrap()
                .2
                .contains(&"--width".into())
        );
        option.version_details.minecraft_arguments = Some("--title 'unclosed".into());
        assert!(option.metadata_arguments("a path").is_err());
        option.version_details.minecraft_arguments = Some("${unknown}".into());
        assert!(
            option
                .metadata_arguments("a path")
                .unwrap_err()
                .to_string()
                .contains("unknown")
        );
        let child = root.join("versions/Fabric");
        fs::create_dir_all(&child).unwrap();
        fs::write(
            child.join("Fabric.json"),
            serde_json::to_vec(&json!({
                "id":"Fabric", "inheritsFrom":"Custom Name", "mainClass":"FabricMain",
                "arguments":{"jvm":["-Dloader=true"],"game":[]},
                "libraries":[{"name":"example:loader:1", "url":"https://example.org/"}]
            }))
            .unwrap(),
        )
        .unwrap();
        fs::create_dir_all(root.join("libraries/example/loader/1")).unwrap();
        let loader = root.join("libraries/example/loader/1/loader-1.jar");
        fs::write(&loader, b"loader").unwrap();
        let inherited = Arc::new(GameInstance::from_version_folder(&child, &repo).unwrap());
        option.version_details = inherited.read_version_json().unwrap();
        option.game_directory = inherited.directory.clone();
        option.game_instance = inherited;
        assert!(option.launch_checked().unwrap().wait().unwrap().success());
        let captured = fs::read_to_string(root.join("captured")).unwrap();
        let lines: Vec<_> = captured.lines().collect();
        assert_eq!(
            fs::canonicalize(lines[0]).unwrap(),
            fs::canonicalize(&child).unwrap()
        );
        assert!(lines.contains(&"FabricMain"));
        assert!(lines.contains(&"-Dloader=true"));
        assert!(
            lines
                .windows(2)
                .any(|pair| pair == ["--username", "Name With Space"])
        );
        let cp = lines.windows(2).find(|pair| pair[0] == "-cp").unwrap()[1];
        assert_eq!(
            std::env::split_paths(cp).collect::<Vec<_>>(),
            [loader, version.join("Custom Name.jar")]
        );
        fs::write(child.join("Fabric.jar"), b"patched").unwrap();
        let forge_metadata = json!({
            "id":"Fabric", "inheritsFrom":"Custom Name", "clientVersion":"1.20.1", "mainClass":"ForgeMain",
            "libraries":[{"name":"net.minecraftforge:forge:1.20.1-47.0.0"}]
        });
        fs::write(
            child.join("Fabric.json"),
            serde_json::to_vec(&forge_metadata).unwrap(),
        )
        .unwrap();
        let forge_library =
            root.join("libraries/net/minecraftforge/forge/1.20.1-47.0.0/forge-1.20.1-47.0.0.jar");
        fs::create_dir_all(forge_library.parent().unwrap()).unwrap();
        fs::write(&forge_library, b"forge").unwrap();
        let mut state = AppState {
            active_account: Some(option.account.clone()),
            active_game_instance: Some(option.game_instance.clone()),
            ..Default::default()
        };
        state.memory_settings = Some(crate::core::memory::MemorySettings::Custom {
            maximum_mib: 4096,
            initial_mib: Some(2048),
        });
        let global_memory = LaunchOption::from_state(&state, option.java_runtime.clone()).unwrap();
        assert_eq!(global_memory.max_memory, 4096);
        assert!(
            global_memory
                .build_jvm_arguments(false)
                .contains(&"-Xms2048m".into())
        );
        state.instance_memory_settings.insert(
            child.clone(),
            crate::core::memory::MemorySettings::Custom {
                maximum_mib: 1536,
                initial_mib: Some(1200),
            },
        );
        let mut java32 = (*option.java_runtime).clone();
        java32.is_64_bit = false;
        let instance_memory = LaunchOption::from_state(&state, Arc::new(java32)).unwrap();
        assert_eq!(
            (instance_memory.max_memory, instance_memory.initial_memory),
            (1024, Some(1024))
        );
        state.window_settings = Some(crate::setup::WindowSettings {
            mode: crate::setup::WindowMode::Custom,
            width: 1280,
            height: 720,
        });
        let custom = LaunchOption::from_state(&state, option.java_runtime.clone()).unwrap();
        assert_eq!((custom.width, custom.height), (Some(1280), Some(720)));
        assert!(!custom.uses_launcher_size());
        assert!(custom.launch_checked().unwrap().wait().unwrap().success());
        assert!(
            fs::read_to_string(root.join("captured"))
                .unwrap()
                .lines()
                .any(|line| line == "-Dwidth=1280")
        );
        state.window_settings.as_mut().unwrap().mode = crate::setup::WindowMode::Launcher;
        assert!(
            LaunchOption::from_state(&state, option.java_runtime.clone())
                .unwrap()
                .uses_launcher_size()
        );
        let option = LaunchOption::from_state(&state, option.java_runtime.clone()).unwrap();
        assert_eq!(option.game_instance.jar_path, child.join("Fabric.jar"));
        assert!(option.launch_checked().unwrap().wait().unwrap().success());
        let captured = fs::read_to_string(root.join("captured")).unwrap();
        let lines: Vec<_> = captured.lines().collect();
        let cp = lines.windows(2).find(|pair| pair[0] == "-cp").unwrap()[1];
        assert_eq!(
            std::env::split_paths(cp).collect::<Vec<_>>(),
            [forge_library, child.join("Fabric.jar")]
        );
        fs::remove_dir_all(root).unwrap();
    }

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
        let classpath = build_classpath(&instance, &details, &RuleContext::current(), &[]).unwrap();
        assert_eq!(
            std::env::split_paths(&classpath).collect::<Vec<_>>(),
            vec![
                root.join("libraries/test/allowed.jar"),
                version.join("Custom.jar")
            ]
        );
        fs::remove_file(root.join("libraries/test/allowed.jar")).unwrap();
        assert!(
            build_classpath(&instance, &details, &RuleContext::current(), &[])
                .unwrap_err()
                .to_string()
                .contains("Missing library")
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn inherited_classpath_prefers_child_and_accepts_local_maven_metadata() {
        let root =
            std::env::temp_dir().join(format!("pcl-inherited-classpath-{}", uuid::Uuid::new_v4()));
        let parent = root.join("versions/base");
        let child = root.join("versions/loader");
        fs::create_dir_all(&parent).unwrap();
        fs::create_dir_all(&child).unwrap();
        fs::write(parent.join("base.jar"), b"client").unwrap();
        let artifact = |path| json!({"sha1":"a".repeat(40), "size":1,"url":"https://example.org/file","path":path});
        fs::write(parent.join("base.json"), serde_json::to_vec(&json!({
            "id":"base","type":"release","mainClass":"Main","assets":"test",
            "downloads":{"client":artifact("client.jar")},"assetIndex":{"id":"test","sha1":"a".repeat(40),"size":1,"url":"https://example.org/index"},
            "libraries":[{"name":"example:shared:1","downloads":{"artifact":artifact("missing-old.jar")}}, {"name":"example:other:1","rules":[{"action":"allow"}],"downloads":{"artifact":artifact("other.jar")}}]
        })).unwrap()).unwrap();
        fs::write(
            child.join("loader.json"),
            serde_json::to_vec(&json!({"id":"loader","inheritsFrom":"base","libraries":[
                {"name":"example:shared:2"},
                {"name":"example:local:1","downloads":{"artifact":{"path":"local.jar"}}},
                {"name":"example:other:2","rules":[{"action":"disallow"}]}
            ]}))
            .unwrap(),
        )
        .unwrap();
        fs::create_dir_all(root.join("libraries/example/shared/2")).unwrap();
        fs::write(
            root.join("libraries/example/shared/2/shared-2.jar"),
            b"loader",
        )
        .unwrap();
        fs::write(root.join("libraries/other.jar"), b"parent").unwrap();
        fs::write(root.join("libraries/local.jar"), b"local").unwrap();
        let instance = GameInstance::from_version_folder(
            &child,
            &super::super::repository::GameRepository::new("test", root.clone()),
        )
        .unwrap();
        let classpath = build_classpath(
            &instance,
            &instance.read_version_json().unwrap(),
            &RuleContext::current(),
            &[],
        )
        .unwrap();
        assert_eq!(
            std::env::split_paths(&classpath).collect::<Vec<_>>(),
            vec![
                root.join("libraries/example/shared/2/shared-2.jar"),
                root.join("libraries/local.jar"),
                root.join("libraries/other.jar"),
                parent.join("base.jar")
            ]
        );
        fs::remove_file(root.join("libraries/example/shared/2/shared-2.jar")).unwrap();
        assert!(
            build_classpath(
                &instance,
                &instance.read_version_json().unwrap(),
                &RuleContext::current(),
                &[]
            )
            .unwrap_err()
            .to_string()
            .contains("Missing library")
        );
        fs::remove_dir_all(root).unwrap();
    }
}
