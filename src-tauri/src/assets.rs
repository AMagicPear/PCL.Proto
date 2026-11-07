//! 本应用的辅助资源路径与 Mesa loader 下载。
//! macOS 使用包内只读资源，其它平台由 core 按需准备默认资源。

use std::path::{Path, PathBuf};

use pcl_core::minecraft::{
    installer::forgelike,
    launch::{launcher::LaunchHelpers, resources},
};

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
    bundled_directory: Option<PathBuf>,
}

impl Assets {
    pub fn new(config_dir: &Path) -> Self {
        Self {
            directory: config_dir.join("resources"),
            bundled_directory: running_bundle().map(|root| root.join("launcher")),
        }
    }

    pub fn forge_installer(&self) -> Option<PathBuf> {
        self.bundled_directory
            .as_ref()
            .map(|directory| directory.join(forgelike::BRIDGE_NAME))
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

    /// 包内文件覆盖 core 默认资源；下载的 Mesa loader 始终放在可写目录。
    pub fn launch_helpers(&self) -> LaunchHelpers {
        let bundled = |name: &str| {
            self.bundled_directory
                .as_ref()
                .map(|directory| directory.join(name))
        };
        LaunchHelpers {
            java_wrapper: bundled(resources::JAVA_WRAPPER_NAME),
            legacy_fix: bundled(resources::LEGACY_FIX_NAME),
            lwjgl_unsafe_agent: bundled(resources::LWJGL_UNSAFE_AGENT_NAME),
            log4j2_debug: bundled(resources::LOG4J2_DEBUG_NAME),
            log4j2_legacy_debug: bundled(resources::LOG4J2_LEGACY_DEBUG_NAME),
            mesa_loader: Some(self.mesa_loader_path()),
            ..Default::default()
        }
    }
}
