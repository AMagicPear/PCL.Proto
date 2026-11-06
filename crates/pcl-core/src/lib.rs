//! PCL.Proto 启动器核心。
//!
//! 这一层只包含实例、安装、启动和网络等领域逻辑，不依赖任何 GUI 工具包，也不读写配置文件；
//! 配置系统、Tauri 命令、窗口和资源协议属于宿主应用。宿主通过注入凭据、资源目录和
//! [`runtime::RuntimePaths`] 来定制行为，并在调用前把配置折算成显式输入。

pub mod app;
pub mod events;
pub mod io;
pub mod link;
pub mod minecraft;
pub mod runtime;
pub mod utils;
