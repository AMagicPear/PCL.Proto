# pcl-core

PCL.Proto 的启动器核心能力，供第三方构建自己的 Minecraft 启动器。

它只做领域逻辑：版本元数据、实例、安装（原版 / Fabric / Forge / NeoForge / 整合包）、
启动、Java 探测与安装、账户认证、下载器、服务器 ping。**不包含**界面、配置文件的读写、
窗口、Tauri 或任何桌面应用资源。

## 它不负责什么

| 你可能期待 | 实际由宿主负责 |
| --- | --- |
| 配置文件路径、读写、迁移 | 自己决定格式与位置，把值折算成下面的显式输入 |
| 全局状态 / 单例 | 没有；所有输入都通过参数传入 |
| 界面、窗口、通知 | 自己实现 |
| 随包的 JAR 资源 | 用 [`pcl-assets`](../pcl-assets) 或自备文件，把路径传进来 |

## 基本用法

```rust,ignore
use pcl_core::io::net::http::api_client::{ApiProvider, MinecraftApiClient};
use pcl_core::runtime::RuntimePaths;

// 1. 注入凭据（没有 CurseForge / 微软登录需求时可以跳过）
pcl_core::secrets::install(pcl_core::secrets::Secrets {
    curseforge_api_key: my_curseforge_key,
    ms_oauth_client_id: my_client_id,
});

// 2. 构造 API 客户端；自定义 User-Agent 只在核心内部构造客户端时使用
pcl_core::io::net::set_user_agent("MyLauncher/1.0");
let client = MinecraftApiClient::new(
    reqwest::Client::builder().user_agent("MyLauncher/1.0").build()?,
    &ApiProvider::Official,
);

// 3. 告诉核心哪些目录可写
let paths = RuntimePaths {
    config_dir: config_dir.clone(),          // authlib-injector.jar、解压资源
    java_runtime_dir: config_dir.join("jre"), // 核心下载的 Java 运行时
};
```

### 安装一个版本

```rust,ignore
use pcl_core::events::event_sink;
use pcl_core::minecraft::installer::{LoaderFamily, LoaderInstall};

let sink = event_sink(|report| { /* 进度 */ });
let loader = LoaderInstall {
    family: LoaderFamily::Fabric,
    version: "0.16.9".into(),
    api: None,
    install_java: false,
};
let java = pcl_core::minecraft::java::JavaState::default();
let outcome = pcl_core::minecraft::installer::download_minecraft_version(
    &minecraft_dir, // 目标 .minecraft
    &client,
    sink,
    "1.21.1".into(),      // 游戏版本
    1,                    // task_id
    "My Instance".into(), // 实例名
    Some(loader),
    &java,
    &paths,
    &forge_bridge_jar,    // 从 pcl-assets 落盘的 forge-installer.jar
)
.await?;
// outcome.directory: 新实例目录；outcome.isolated: 是否隔离
```

### 启动游戏

启动输入由宿主组装；核心只执行。

```rust,ignore
use pcl_core::minecraft::launch::launcher::{LaunchOption, LaunchRequest};

let option = LaunchOption::new(LaunchRequest {
    paths,
    brand: LauncherBrand { name: "MyLauncher".into(), version: "1.0".into() },
    instance,            // Arc<McInstance>
    profile,             // McProfile
    java: java_entry,    // Arc<JavaEntry>
    game_directory,
    max_memory: 4096,
    initial_memory: None,
    width: None, height: None, launcher_size: false,
    arguments,           // LaunchArguments
    extras,              // LaunchExtras：代理、预启动命令、辅助功能开关
    helpers,             // LaunchHelpers：辅助 JAR 路径，缺省即跳过对应功能
})?;
option.prepare_natives(&downloader).await?;
option.launch_checked()?;
```

### Java 列表

```rust,ignore
let state = pcl_core::minecraft::java::refresh(
    true,
    &JavaState { runtimes, selected },
    &JavaRoots { repositories, runtime_dir },
)
.await?;
// 结果由宿主持久化；核心不写配置。
```

## Feature

| feature | 默认 | 作用 |
| --- | --- | --- |
| `link` | 关闭 | 引入 Terracotta 联机桥接（`pcl_core::link::scaffolding`） |

`link` 目前依赖同仓的 `crates/terracotta`（Git 子模块）。**它还没有发布到 crates.io**，
所以开启该 feature 的 `pcl-core` 暂时不能直接发布；只用单机能力时保持默认 feature 即可。

## 配套 crate

- [`pcl-assets`](../pcl-assets)：Java wrapper、LegacyFix、Log4j 配置、Forge 安装桥接、
  macOS ARM64 库映射。核心不依赖它，需要这些行为时自己落盘并传路径。

## 许可

见仓库根目录 `LICENSE`。
