//! 启动器运行时的目录约定。由宿主应用提供，核心库只读取路径，不关心它们从哪来。

use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct RuntimePaths {
    /// 可写的启动器配置目录：authlib-injector.jar、解压出来的辅助资源等。
    pub config_dir: PathBuf,
    /// 启动器自己安装的 Java 运行时目录。
    pub java_runtime_dir: PathBuf,
}
