//! Per-instance launch settings that CE keeps outside the plain launch arguments.
#[cfg(not(target_os = "macos"))]
use std::io::Write;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::runtime::RuntimePaths;

/// 0 follows the global renderer, 1..=4 map to the global numbering 0..=3.
pub const RENDERER_FOLLOW_GLOBAL: u8 = 0;
pub const RENDERER_MAX: u8 = 4;
pub const MESA_LOADER_VERSION: &str = "26.0.4";

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InstanceLaunchSettings {
    /// Custom version info shown in the game; empty follows the global value.
    pub info: String,
    pub renderer: u8,
    pub prelaunch_command: String,
    pub prelaunch_wait: bool,
    pub disable_asset_verify: bool,
    pub disable_jlw: bool,
    pub disable_legacy_fix: bool,
    pub debug_log4j2: bool,
    pub disable_lwjgl_unsafe_agent: bool,
    pub use_proxy: bool,
}

impl InstanceLaunchSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.renderer > RENDERER_MAX {
            return Err("Invalid renderer choice".into());
        }
        validate_info(&self.info)?;
        if self.prelaunch_command.len() > 16384 || self.prelaunch_command.contains('\0') {
            return Err("Invalid instance launch setting".into());
        }
        Ok(())
    }
}

/// Effective values after merging global defaults with the instance override.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LaunchExtras {
    pub info: String,
    /// 0 = game default, 1 = llvmpipe, 2 = d3d12, 3 = zink.
    pub renderer: u8,
    pub prelaunch_command: String,
    pub prelaunch_wait: bool,
    pub disable_asset_verify: bool,
    pub java_wrapper: bool,
    pub legacy_fix: bool,
    pub debug_log4j2: bool,
    pub lwjgl_unsafe_agent: bool,
    pub proxy: Option<(String, u16)>,
}

/// 全局启动开关；实例设置只在未覆盖时回落到这里。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GlobalLaunchSwitches {
    pub version_info: String,
    pub renderer: u8,
    pub disable_jlw: bool,
    pub disable_legacy_fix: bool,
    pub disable_lwjgl_unsafe_agent: bool,
}

pub fn resolve(
    global: &GlobalLaunchSwitches,
    instance_settings: Option<&InstanceLaunchSettings>,
) -> LaunchExtras {
    let settings = instance_settings.cloned().unwrap_or_default();
    // Empty info stays empty; the launcher drops `--versionType` entirely, as CE does.
    let info = if settings.info.trim().is_empty() {
        global.version_info.clone()
    } else {
        settings.info.clone()
    };
    let renderer = if settings.renderer == RENDERER_FOLLOW_GLOBAL {
        global.renderer
    } else {
        settings.renderer - 1
    }
    .min(3);
    let proxy = settings
        .use_proxy
        .then(crate::io::net::proxy::current)
        .and_then(|settings| settings.game_endpoint());
    LaunchExtras {
        info,
        renderer,
        prelaunch_command: settings.prelaunch_command,
        prelaunch_wait: settings.prelaunch_wait,
        disable_asset_verify: settings.disable_asset_verify,
        java_wrapper: !global.disable_jlw && !settings.disable_jlw,
        legacy_fix: !global.disable_legacy_fix && !settings.disable_legacy_fix,
        debug_log4j2: settings.debug_log4j2,
        lwjgl_unsafe_agent: !global.disable_lwjgl_unsafe_agent
            && !settings.disable_lwjgl_unsafe_agent,
        proxy,
    }
}

/// CE's `LaunchArgumentInfo` rule: bounded length, no NUL, and no quotes, which would
/// otherwise leak into the `--versionType` argument.
pub fn validate_info(value: &str) -> Result<(), String> {
    if value.len() > 16384
        || value
            .chars()
            .any(|c| matches!(c, '\0' | '"' | '\u{201c}' | '\u{201d}'))
    {
        return Err("Invalid version info".into());
    }
    Ok(())
}

/// CE only uses the Java wrapper for legacy versions under a UTF-8 Windows code page.
pub fn java_wrapper_supported() -> bool {
    if crate::utils::platform::host_architecture() == "aarch64" {
        return false;
    }
    #[cfg(windows)]
    {
        // SAFETY: GetACP has no preconditions.
        unsafe { windows_sys::Win32::Globalization::GetACP() == 65001 }
    }
    #[cfg(not(windows))]
    {
        false
    }
}

fn resource_dir(paths: &RuntimePaths) -> PathBuf {
    paths.config_dir.join("resources")
}

#[cfg(not(target_os = "macos"))]
fn bundled(paths: &RuntimePaths, name: &str, bytes: &[u8]) -> std::io::Result<PathBuf> {
    let directory = resource_dir(paths);
    let path = directory.join(name);
    if std::fs::metadata(&path).is_ok_and(|metadata| metadata.len() == bytes.len() as u64) {
        return Ok(path);
    }
    std::fs::create_dir_all(&directory)?;
    let temp = path.with_extension(format!("{}.tmp", std::process::id()));
    let mut file = std::fs::File::create(&temp)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    drop(file);
    std::fs::rename(&temp, &path)?;
    Ok(path)
}

macro_rules! bundled_resource {
    ($paths:expr, $name:literal) => {{
        #[cfg(target_os = "macos")]
        {
            let _ = $paths;
            crate::app::resources::launcher($name)
        }
        #[cfg(not(target_os = "macos"))]
        {
            bundled(
                $paths,
                $name,
                include_bytes!(concat!("../../../resources/", $name)),
            )
        }
    }};
}

pub fn java_wrapper(paths: &RuntimePaths) -> std::io::Result<PathBuf> {
    bundled_resource!(paths, "java-wrapper.jar")
}

pub fn forge_installer(paths: &RuntimePaths) -> std::io::Result<PathBuf> {
    bundled_resource!(paths, "forge-installer.jar")
}

pub fn legacy_fix(paths: &RuntimePaths) -> std::io::Result<PathBuf> {
    bundled_resource!(paths, "legacyfix.jar")
}

pub fn lwjgl_unsafe_agent(paths: &RuntimePaths) -> std::io::Result<PathBuf> {
    bundled_resource!(paths, "lwjgl-unsafe-agent.jar")
}

pub fn log4j2_config(paths: &RuntimePaths, legacy: bool) -> std::io::Result<PathBuf> {
    if legacy {
        bundled_resource!(paths, "log4j2-legacy-debug.xml")
    } else {
        bundled_resource!(paths, "log4j2-debug.xml")
    }
}

pub fn mesa_loader_path(paths: &RuntimePaths) -> PathBuf {
    resource_dir(paths)
        .join("mesa-loader-windows")
        .join(MESA_LOADER_VERSION)
        .join("Loader.jar")
}

pub fn mesa_loader_url() -> String {
    let arch = match crate::utils::platform::host_architecture() {
        "x86" => "x86",
        "aarch64" => "arm64",
        _ => "x64",
    };
    format!(
        "https://mirrors.cloud.tencent.com/nexus/repository/maven-public/org/glavo/mesa-loader-windows/{MESA_LOADER_VERSION}/mesa-loader-windows-{MESA_LOADER_VERSION}-{arch}.jar"
    )
}

/// Download the renderer agent CE fetches from its mirror; there is no hash to verify.
pub async fn ensure_mesa_loader(paths: &RuntimePaths) -> anyhow::Result<PathBuf> {
    let path = mesa_loader_path(paths);
    if path.is_file() {
        return Ok(path);
    }
    let parent = path.parent().expect("mesa loader path has a parent");
    std::fs::create_dir_all(parent)?;
    let url = mesa_loader_url();
    log::info!("Downloading renderer agent from {url}");
    let response = crate::io::download::send_request(
        crate::io::download::resources::HttpMode::Auto,
        true,
        &url,
        None,
        Some(std::time::Duration::from_secs(120)),
    )
    .await?
    .error_for_status()?;
    let bytes = response.bytes().await?;
    anyhow::ensure!(
        (1024..=64 * 1024 * 1024).contains(&bytes.len()),
        "Invalid renderer agent size"
    );
    let temp = path.with_extension(format!("{}.tmp", std::process::id()));
    tokio::fs::write(&temp, &bytes).await?;
    tokio::fs::rename(&temp, &path).await?;
    Ok(path)
}

/// Run the pre-launch command like CE: failures are logged, never fatal.
pub fn run_prelaunch(extras: &LaunchExtras, working_directory: &Path) {
    let command = extras.prelaunch_command.trim();
    if command.is_empty() {
        return;
    }
    let mut process = if cfg!(windows) {
        let mut process = Command::new("cmd");
        process.arg("/c").arg(command);
        process
    } else {
        let mut process = Command::new("sh");
        process.arg("-c").arg(command);
        process
    };
    process
        .current_dir(working_directory)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    match process.spawn() {
        Ok(mut child) => {
            if extras.prelaunch_wait
                && let Err(error) = child.wait()
            {
                log::warn!("Pre-launch command did not finish: {error}");
            }
        }
        Err(error) => log::warn!("Cannot run pre-launch command: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn global() -> GlobalLaunchSwitches {
        GlobalLaunchSwitches {
            version_info: "PCL.Proto".into(),
            ..Default::default()
        }
    }

    #[test]
    fn resolves_global_defaults_and_instance_overrides() {
        let mut global = global();
        let mut settings = InstanceLaunchSettings {
            renderer: RENDERER_FOLLOW_GLOBAL,
            use_proxy: true,
            disable_jlw: true,
            ..Default::default()
        };
        let extras = resolve(&global, Some(&settings));
        assert_eq!(extras.info, "PCL.Proto");
        assert_eq!(extras.renderer, 0);
        assert!(!extras.java_wrapper);
        assert_eq!(extras.proxy, None);
        assert!(extras.legacy_fix);
        assert!(extras.lwjgl_unsafe_agent);

        global.renderer = 2;
        let extras = resolve(&global, Some(&settings));
        assert_eq!(extras.renderer, 2);
        settings.renderer = 4;
        let extras = resolve(&global, Some(&settings));
        assert_eq!(extras.renderer, 3);
        assert_eq!(resolve(&global, None).info, "PCL.Proto");
        global.version_info = String::new();
        assert!(resolve(&global, None).info.is_empty());
    }

    #[test]
    fn validates_renderer_and_command_bounds() {
        assert!(
            InstanceLaunchSettings {
                renderer: RENDERER_MAX + 1,
                ..Default::default()
            }
            .validate()
            .is_err()
        );
        assert!(
            InstanceLaunchSettings {
                prelaunch_command: "a\0b".into(),
                ..Default::default()
            }
            .validate()
            .is_err()
        );
        assert!(InstanceLaunchSettings::default().validate().is_ok());
        assert!(
            InstanceLaunchSettings {
                info: "PCL \"Proto\"".into(),
                ..Default::default()
            }
            .validate()
            .is_err()
        );
        assert!(validate_info("PCL.Proto").is_ok());
    }

    #[test]
    fn skips_missing_prelaunch_commands_without_failing() {
        let extras = LaunchExtras {
            prelaunch_command: "   ".into(),
            ..Default::default()
        };
        run_prelaunch(&extras, Path::new("."));
    }
}
