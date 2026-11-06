//! PCL.Proto 的可选启动器资源。
//!
//! 这些文件不是 `pcl-core` 的依赖：需要 CE 兼容的启动补丁或 Forge 安装桥接时，
//! 宿主把字节落到自己选择的目录，再把路径传给 `pcl-core`。
//!
//! ```no_run
//! let dir = std::env::temp_dir().join("my-launcher/resources");
//! pcl_assets::materialize(&dir).unwrap();
//! let forge_bridge = dir.join(pcl_assets::FORGE_INSTALLER_NAME);
//! ```

use std::path::{Path, PathBuf};

pub const JAVA_WRAPPER_NAME: &str = "java-wrapper.jar";
pub const LEGACY_FIX_NAME: &str = "legacyfix.jar";
pub const LWJGL_UNSAFE_AGENT_NAME: &str = "lwjgl-unsafe-agent.jar";
pub const LOG4J2_DEBUG_NAME: &str = "log4j2-debug.xml";
pub const LOG4J2_LEGACY_DEBUG_NAME: &str = "log4j2-legacy-debug.xml";
pub const FORGE_INSTALLER_NAME: &str = "forge-installer.jar";
pub const MACOS_ARM_LIBRARIES_NAME: &str = "macos-arm64-libraries.json";

pub const JAVA_WRAPPER: &[u8] = include_bytes!("../resources/java-wrapper.jar");
pub const LEGACY_FIX: &[u8] = include_bytes!("../resources/legacyfix.jar");
pub const LWJGL_UNSAFE_AGENT: &[u8] = include_bytes!("../resources/lwjgl-unsafe-agent.jar");
pub const LOG4J2_DEBUG: &[u8] = include_bytes!("../resources/log4j2-debug.xml");
pub const LOG4J2_LEGACY_DEBUG: &[u8] = include_bytes!("../resources/log4j2-legacy-debug.xml");
pub const FORGE_INSTALLER: &[u8] = include_bytes!("../resources/forge-installer.jar");
pub const MACOS_ARM_LIBRARIES: &str = include_str!("../resources/macos-arm64-libraries.json");

/// 一份命名资源；供批量落盘使用。
pub struct Asset {
    pub name: &'static str,
    pub bytes: &'static [u8],
}

pub const ALL: &[Asset] = &[
    Asset {
        name: JAVA_WRAPPER_NAME,
        bytes: JAVA_WRAPPER,
    },
    Asset {
        name: LEGACY_FIX_NAME,
        bytes: LEGACY_FIX,
    },
    Asset {
        name: LWJGL_UNSAFE_AGENT_NAME,
        bytes: LWJGL_UNSAFE_AGENT,
    },
    Asset {
        name: LOG4J2_DEBUG_NAME,
        bytes: LOG4J2_DEBUG,
    },
    Asset {
        name: LOG4J2_LEGACY_DEBUG_NAME,
        bytes: LOG4J2_LEGACY_DEBUG,
    },
    Asset {
        name: FORGE_INSTALLER_NAME,
        bytes: FORGE_INSTALLER,
    },
    Asset {
        name: MACOS_ARM_LIBRARIES_NAME,
        bytes: MACOS_ARM_LIBRARIES.as_bytes(),
    },
];

/// 把全部资源写到 `directory`；大小一致的已有文件会跳过。
pub fn materialize(directory: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(directory)?;
    for asset in ALL {
        write_if_changed(directory, asset.name, asset.bytes)?;
    }
    Ok(())
}

/// 写入单个资源并返回最终路径。
pub fn write(directory: &Path, name: &str, bytes: &[u8]) -> std::io::Result<PathBuf> {
    std::fs::create_dir_all(directory)?;
    write_if_changed(directory, name, bytes)
}

fn write_if_changed(directory: &Path, name: &str, bytes: &[u8]) -> std::io::Result<PathBuf> {
    let path = directory.join(name);
    if std::fs::metadata(&path).is_ok_and(|metadata| metadata.len() == bytes.len() as u64) {
        return Ok(path);
    }
    let temp = path.with_extension(format!("{}.tmp", std::process::id()));
    std::fs::write(&temp, bytes)?;
    std::fs::rename(&temp, &path)?;
    Ok(path)
}
