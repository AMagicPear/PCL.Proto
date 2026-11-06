pub mod mcping;
#[cfg(feature = "link")]
pub mod scaffolding;

/// 初始化联机库。未启用 `link` feature 时不做任何事，宿主仍然可以正常启动。
pub fn init(identifier_path: std::path::PathBuf) {
    #[cfg(feature = "link")]
    terracotta::init_lib(identifier_path);
    #[cfg(not(feature = "link"))]
    let _ = identifier_path;
}
