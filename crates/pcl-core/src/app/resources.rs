//! 启动器自带的只读资源（JAR 辅助程序、Log4j 配置、ARM64 库映射等）。
//!
//! 未打包运行（`tauri dev`、测试或第三方集成）时使用 crate 自带的 `resources` 目录；
//! 打包后从可执行文件所在的 `.app/Contents/Resources/launcher` 读取，宿主也可以显式注入。

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static LAUNCHER_DIR: OnceLock<PathBuf> = OnceLock::new();

/// 由宿主应用在启动早期注入资源目录；重复注入时保留第一次的值。
pub fn set_launcher_dir(directory: PathBuf) {
    let _ = LAUNCHER_DIR.set(directory);
}

/// 从可执行文件路径推导 `.app` 包的 `Contents` 目录。
pub fn bundle_root(executable: &Path) -> Option<PathBuf> {
    let macos = executable.parent()?;
    let contents = macos.parent()?;
    (macos.file_name()? == "MacOS" && contents.file_name()? == "Contents")
        .then(|| contents.join("Resources"))
}

fn launcher_dir() -> PathBuf {
    if let Some(directory) = LAUNCHER_DIR.get() {
        return directory.clone();
    }
    let bundle = std::env::current_exe()
        .ok()
        .and_then(|executable| executable.canonicalize().ok())
        .and_then(|executable| bundle_root(&executable));
    match bundle {
        // 打包后从 .app 的只读资源目录读取。
        Some(root) => root.join("launcher"),
        // 未打包（tauri dev、测试或第三方集成）时使用 crate 自带的资源目录。
        None => Path::new(env!("CARGO_MANIFEST_DIR")).join("resources"),
    }
}

/// 返回一个已确认存在的启动器资源路径。
pub fn launcher(name: &str) -> std::io::Result<PathBuf> {
    let path = launcher_dir().join(name);
    std::fs::File::open(&path)?;
    Ok(path)
}
