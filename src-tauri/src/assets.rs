//! 启动器随包携带的辅助资源：Java wrapper、LegacyFix、渲染器代理等。
//!
//! 打包后从 `.app/Contents/Resources/launcher` 读取；其它平台把它们嵌进二进制，
//! 首次使用时解压到配置目录。核心库只接收最终路径，不关心这些文件从哪来。

use std::path::{Path, PathBuf};

use pcl_core::minecraft::launch::launcher::LaunchHelpers;

/// 渲染器代理的版本，对照 CE 的 mesa-loader-windows。
pub const MESA_LOADER_VERSION: &str = "26.0.4";

/// 从可执行文件路径推导 `.app` 包的 `Contents/Resources`。
pub fn bundle_root(executable: &Path) -> Option<PathBuf> {
    let macos = executable.parent()?;
    let contents = macos.parent()?;
    (macos.file_name()? == "MacOS" && contents.file_name()? == "Contents")
        .then(|| contents.join("Resources"))
}

fn running_bundle() -> Option<PathBuf> {
    let executable = std::env::current_exe().ok()?.canonicalize().ok()?;
    bundle_root(&executable)
}

pub struct Assets {
    directory: PathBuf,
    /// 打包后直接在应用包里取文件，不再往只读目录写。
    bundled: bool,
}

impl Assets {
    pub fn new(config_dir: &Path) -> Self {
        match running_bundle() {
            Some(root) => Self {
                directory: root.join("launcher"),
                bundled: true,
            },
            None => Self {
                directory: config_dir.join("resources"),
                bundled: false,
            },
        }
    }

    fn path(&self, name: &str, bytes: &[u8]) -> std::io::Result<PathBuf> {
        let path = self.directory.join(name);
        if self.bundled {
            return Ok(path);
        }
        if std::fs::metadata(&path).is_ok_and(|metadata| metadata.len() == bytes.len() as u64) {
            return Ok(path);
        }
        std::fs::create_dir_all(&self.directory)?;
        let temp = path.with_extension(format!("{}.tmp", std::process::id()));
        std::fs::write(&temp, bytes)?;
        std::fs::rename(&temp, &path)?;
        Ok(path)
    }

    pub fn java_wrapper(&self) -> std::io::Result<PathBuf> {
        self.path(pcl_assets::JAVA_WRAPPER_NAME, pcl_assets::JAVA_WRAPPER)
    }

    pub fn forge_installer(&self) -> std::io::Result<PathBuf> {
        self.path(
            pcl_assets::FORGE_INSTALLER_NAME,
            pcl_assets::FORGE_INSTALLER,
        )
    }

    pub fn legacy_fix(&self) -> std::io::Result<PathBuf> {
        self.path(pcl_assets::LEGACY_FIX_NAME, pcl_assets::LEGACY_FIX)
    }

    pub fn lwjgl_unsafe_agent(&self) -> std::io::Result<PathBuf> {
        self.path(
            pcl_assets::LWJGL_UNSAFE_AGENT_NAME,
            pcl_assets::LWJGL_UNSAFE_AGENT,
        )
    }

    fn log4j2_debug(&self) -> std::io::Result<PathBuf> {
        self.path(pcl_assets::LOG4J2_DEBUG_NAME, pcl_assets::LOG4J2_DEBUG)
    }

    fn log4j2_legacy_debug(&self) -> std::io::Result<PathBuf> {
        self.path(
            pcl_assets::LOG4J2_LEGACY_DEBUG_NAME,
            pcl_assets::LOG4J2_LEGACY_DEBUG,
        )
    }

    fn macos_arm_libraries(&self) -> Option<String> {
        Some(pcl_assets::MACOS_ARM_LIBRARIES.to_owned())
    }

    pub fn mesa_loader_path(&self) -> PathBuf {
        self.directory
            .join("mesa-loader-windows")
            .join(MESA_LOADER_VERSION)
            .join("Loader.jar")
    }

    /// 下载渲染器代理；CE 的镜像没有校验值。
    pub async fn ensure_mesa_loader(&self) -> Result<PathBuf, String> {
        let path = self.mesa_loader_path();
        if path.is_file() {
            return Ok(path);
        }
        let parent = path.parent().ok_or("Invalid renderer agent path")?;
        std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        let arch = match pcl_core::utils::platform::host_architecture() {
            "x86" => "x86",
            "aarch64" => "arm64",
            _ => "x64",
        };
        let url = format!(
            "https://mirrors.cloud.tencent.com/nexus/repository/maven-public/org/glavo/mesa-loader-windows/{MESA_LOADER_VERSION}/mesa-loader-windows-{MESA_LOADER_VERSION}-{arch}.jar"
        );
        log::info!("Downloading renderer agent from {url}");
        let response = crate::config::manager::ConfigManager::instance()
            .api_client
            .downloader()
            .await
            .send_request(
                pcl_core::io::download::resources::HttpMode::Auto,
                true,
                &url,
                None,
                Some(std::time::Duration::from_secs(120)),
            )
            .await
            .map_err(|error| error.to_string())?
            .error_for_status()
            .map_err(|error| error.to_string())?;
        let bytes = response.bytes().await.map_err(|error| error.to_string())?;
        if !(1024..=64 * 1024 * 1024).contains(&bytes.len()) {
            return Err("Invalid renderer agent size".into());
        }
        let temp = path.with_extension(format!("{}.tmp", std::process::id()));
        tokio::fs::write(&temp, &bytes)
            .await
            .map_err(|error| error.to_string())?;
        tokio::fs::rename(&temp, &path)
            .await
            .map_err(|error| error.to_string())?;
        Ok(path)
    }

    /// 汇总成启动流程需要的辅助文件。
    pub fn launch_helpers(&self) -> LaunchHelpers {
        LaunchHelpers {
            java_wrapper: self.java_wrapper().ok(),
            legacy_fix: self.legacy_fix().ok(),
            lwjgl_unsafe_agent: self.lwjgl_unsafe_agent().ok(),
            log4j2_debug: self.log4j2_debug().ok(),
            log4j2_legacy_debug: self.log4j2_legacy_debug().ok(),
            macos_arm_libraries: self.macos_arm_libraries(),
            mesa_loader: Some(self.mesa_loader_path()),
        }
    }
}
