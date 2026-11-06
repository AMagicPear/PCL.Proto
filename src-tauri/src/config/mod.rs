//! 启动器配置：文件位置、读写、内存状态与档案持久化。
//!
//! 领域逻辑在 `pcl-core`，这一层只负责"配置从哪里来、存到哪里去"。

pub mod arguments;
pub mod isolation;
pub mod java;
pub mod launch;
pub mod launch_settings;
pub mod manager;
pub mod metadata;
pub mod profiles;
pub mod states;

pub use java::active_instance as current_instance;
pub use states::WindowSettings;
