# pcl-core

用于构建 Minecraft 启动器的 Rust 库：版本、实例、安装、Java、认证、启动、下载、导出、存档与服务器文件、服务器 ping。core 不依赖 Tauri、`pcl-assets` 或 PCL.Proto 的配置系统。

## 通过 Git 引入

只通过 Git 分发，不发布到 crates.io。固定 `rev` 或 `tag`：

```toml
[dependencies]
pcl-core = { git = "https://github.com/PCL-Community/PCL.Proto", rev = "<commit>" }
reqwest = "0.13"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }

# 需要辅助 JAR 或 Forge 处理器桥接时再引入，也可以自行提供文件：
pcl-assets = { git = "https://github.com/PCL-Community/PCL.Proto", rev = "<commit>" }
```

联机默认关闭。需要时在 `pcl-core` 依赖上添加 `features = ["link"]`，才会编译 Terracotta；服务器 ping 不需要该 feature。
Terracotta 来自同仓 Git 子模块，使用 AGPL-3.0；使用 Git CLI/镜像时也须能访问子模块地址。

## 网络与下载

宿主负责代理、User-Agent、超时及凭据来源，core 接收可使用的客户端：

```rust,no_run
use pcl_core::io::{
    download::{Downloader, resources::DownloadOptions},
    net::http::api_client::{ApiProvider, MinecraftApiClient},
};

let build = || reqwest::Client::builder()
    .user_agent("MyLauncher/1.0")
    .connect_timeout(std::time::Duration::from_secs(15))
    .read_timeout(std::time::Duration::from_secs(15));
let downloader = Downloader::new(build().build()?, build().http1_only().build()?);
downloader.resources().configure(DownloadOptions::default())?;
let api = MinecraftApiClient::new(downloader.clone(), &ApiProvider::Official);
# Ok::<(), Box<dyn std::error::Error>>(())
```

也可以用 `Downloader::default()` 的中性 UA 和默认 HTTP 客户端。`clone()` 共享连接配额、限速、统计和客户端；重新构造则隔离。`api.downloader().await` 同时沿用 API 当前的来源地址。
`replace_clients(client, http1)` 让后续请求使用新客户端，已发出的请求不受影响。

CurseForge key 传给 `api.fetch_with_curseforge(endpoint, key)`，不会注入全局状态。下载选项只控制网络功能，不包含安装后选中实例等界面策略。

## 安装

先创建目标 `.minecraft` 目录。安装器接收按优先级排列的 Java 候选，实际使用前重新探测兼容性；无需处理器的安装可传空列表：

```rust,ignore
use pcl_core::{events::event_sink, minecraft::installer};

let outcome = installer::install_version(
    &api,
    &minecraft_dir,
    "1.21.1",
    "My Instance",
    None,                        // 不安装加载器
    1,                           // 进程内唯一 task_id
    &event_sink(|report| { /* 进度 */ }),
    &[],                         // Java 候选
    &data_dir.join("java"),       // 自动安装 Java 的目标目录
    None,                        // Forge 处理器桥接，仅需要时提供 Some(path)
).await?;
// outcome.directory：已发布的实例；outcome.isolated：安装结果是否需要隔离
```

需要辅助资源时使用 `pcl_assets::materialize(&resource_dir)`，或只写入需要的资源，再传具体文件路径。Forge/NeoForge 处理器需要 `forge-installer.jar`；原版和 Fabric 不依赖资源 crate。

## 启动

宿主自行决定全局/实例继承、Java 选择、内存分配、隔离、窗口尺寸和辅助功能开关，再构造一次启动的输入：

```rust,ignore
use pcl_core::minecraft::launch::{
    arguments::LaunchArguments,
    launcher::{LaunchHelpers, LaunchOption, LaunchRequest, LauncherBrand},
    settings::LaunchExtras,
};

let option = LaunchOption::new(LaunchRequest {
    wrapper_temp_dir: data_dir.join("java-wrapper-tmp"),
    brand: LauncherBrand { name: "MyLauncher".into(), version: "1.0".into() },
    instance, profile, java, game_directory,
    max_memory: 4096, initial_memory: None,
    width: None, height: None,
    arguments: LaunchArguments::default(),
    extras: LaunchExtras::default(),
    helpers: LaunchHelpers::default(),
})?;
let downloader = api.downloader().await;
option.prepare_natives(&downloader).await?;
option.prepare_legacy_assets(&downloader).await?;
let child = option.launch_checked()?;
```

`LaunchArguments` 只包含实际 JVM/游戏参数和 classpath，不处理继承。`LaunchHelpers` 中缺省的文件会跳过对应辅助行为；core 不寻找 PCL.Proto 的资源目录。
第三方账户先调用 `launch::authlib::prepare(&client, &profile, &jar_path)` 并将返回的 agent 交给 `with_authlib`。

## Java 与认证

- `minecraft::java::scan(known, roots).await` 扫描系统 Java 和指定目录。`JavaManager` 合并列表，`JavaRequirements` 判断版本兼容性；选择偏好、相对启动器目录与扫描节流由宿主负责。
- `minecraft::java::install::install(requirements, runtime_dir, downloader, progress)` 安装 Mojang Java，不写启动器配置。
- `MicrosoftProvider::new(client, client_id)`、`AuthlibProvider::new(client, server)`、`Connect::new(client, discovery, client_id, server)` 接收自己的客户端与参数。设备码交互、角色选择、档案持久化由宿主编排。
- Connect 未指定 Client ID 时可使用服务发现文档的共享 ID；PCL 在皮肤站注册的 ID 留在应用，第三方应提供自己的注册信息。

## 运行状态与边界

任务取消、游戏会话、在线整合包暂存和 Terracotta 桥接仍使用进程级运行状态，任务 ID 应唯一。HTTP 客户端、配额、代理与凭据不依赖进程级配置。`EventSink` 是普通线程安全回调，宿主可接任意 GUI/CLI。

这里的可编译 API 与本地回归不代表真实游戏、账户服务或 Windows/Linux 桌面已经验收。应用维护与验证命令见 [架构文档](../../docs/ARCHITECTURE.md)。许可见仓库根目录 `LICENSE`。
