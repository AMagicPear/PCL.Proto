//! PCL.Proto 启动器核心。
//!
//! 这一层只包含配置、实例、安装、启动和网络等领域逻辑，不依赖任何 GUI 工具包；
//! Tauri 命令、窗口和资源协议属于宿主应用，宿主通过注入凭据与资源目录来定制行为。

pub mod app;
pub mod events;
pub mod io;
pub mod link;
pub mod minecraft;
pub mod utils;

pub use app::config::{AppState, CONFIG_MANAGER, ConfigManager};
