# PCL2-CE 后端迁移记录

更新：2026-10-29。功能参考 `../PCL2-CE`，revision `d68819990da092ed7e582b94b950874fa30baffe`。

本记录区分“代码已接通”“离线回归通过”“官方服务验证通过”和“真实游戏启动通过”。原版安装通过不代表游戏启动、加载器或账户认证已经完成。

## 迁移原则

先遵循 Rust 与 Vue 3 的惯用方式，再对齐 CE 的用户功能、阶段名称与文件约定。直接迁移行为契约，不复刻 WPF、VB/C# 的线程模型或全局状态。Rust 服务负责下载、校验、文件发布与错误；Tauri 命令只适配参数和状态；Pinia 管理界面任务，命令返回值决定任务最终成功或失败。

## 第一批：原版安装与共享下载

对应 CE 源文件：

- `Pages/PageDownload/ModDownloadLib.cs` 的 `McDownloadClientLoader`：元数据、支持库、资源安装阶段。
- `Modules/Minecraft/ModLibrary.cs`：支持库规则、本地库与平台选择。
- `Modules/Minecraft/ModAssets.cs`：资源索引、散列对象、旧资源布局。

Proto 实现：

| 文件 | 职责 |
| --- | --- |
| `src-tauri/src/core/minecraft.rs` | Mojang 元数据类型、有序规则、平台上下文、本地库 classifier、共享支持库路径 |
| `src-tauri/src/core/api_client.rs` | manifest v2、版本 JSON 的 SHA-1/ID 校验、保留完整 JSON、官方地址与 BMCLAPI 地址转换 |
| `src-tauri/src/core/downloader.rs` | 单文件流式传输、大小与 SHA-1 校验、缓存校验、重试、临时文件发布、文件与阶段进度 |
| `src-tauri/src/core/installer.rs` | 安装计划、并发下载、natives 解压、旧资源映射、实例发布与失败清理 |
| `src-tauri/src/core/version_export.rs` | 客户端/服务端导出，复用相同下载器与经校验的版本元数据 |
| `src-tauri/src/core/repository.rs`、`game.rs` | 按需扫描、精确识别实例 JSON、自定义名称与原版版本号分离 |
| `src-tauri/src/commands.rs` | 选择安装文件夹、扫描与选择实例的 IPC，拒绝越界索引和不存在的实例 |
| `src/stores/task.ts` | Vue 响应式任务、IPC 任务隔离、终态与速度管理、错误保留 |

已改变的行为：

1. 不再依赖“先点击版本、再从系统临时目录复制 JSON”才能安装。每次安装从已验证的 manifest 元数据开始，完整保留 `arguments`、`javaVersion` 等字段。
2. 下载最多尝试三次，网络、流中断及校验失败会传播到调用者；普通永久性 HTTP 4xx 不重试，408/429 可以重试。连接和读取有超时。
3. 只有大小及 SHA-1 均正确的本地文件才被复用。下载写入同目录唯一 `.part` 文件，校验后再 rename；失败不覆盖原文件，取消下载 future 时清理其临时文件。
4. 库规则按顺序执行，最后匹配的 `allow`/`disallow` 生效；支持无 OS 条件、OS 名称、架构/版本正则及功能条件。非法正则返回错误。
5. 下载当前平台的 artifact 和旧版 native classifier；classifier-only 库不再导致解析失败。解压遵循 `extract.exclude`，不释放 `META-INF`，拒绝目录越界及符号链接。
6. 资源按散列去重，验证散列及逻辑路径。支持 `virtual` 与 `map_to_resources`，日志配置放入 `assets/log_configs`。第三方文件来源不被错误替换为 BMCLAPI。
7. 新实例目录必须由本次操作独占创建；已有目录不覆盖。客户端 JAR 和 JSON 使用实例名，JSON 的 `clientVersion` 保留来源版本。全部阶段成功后才发布正式 JSON；失败清理本次新建的实例目录，已校验的共享缓存保留。
8. 安装与启动共用支持库规则和路径；classpath 使用系统分隔符，缺失库直接报错。启动侧改用实际下载的日志配置。
9. 重新进入实例列表会重新扫描。选择 Minecraft 文件夹会同步后端安装目录，即使该文件夹还没有实例。前端仓库字段统一为 `repositories`。
10. 文件全部完成后，任务仍等待安装命令完成；解压或发布失败不会显示安装成功。错误保留在任务页，终态任务不再贡献速度或剩余文件数，迟到的 IPC 不会重启任务。阶段进度按时间节流，缓存命中不计入网络速度。

任务阶段保持现有数值协议，文案引用 CE 语言资源：

| ID | 安装阶段 |
| --- | --- |
| 0 | 下载原版 Json 文件 |
| 1 | 下载核心 Jar 文件 |
| 2 | 下载原版支持库文件 |
| 3 | 下载原版资源文件 |

## 验证记录

- `cargo check -p pcl-proto --lib` 通过。
- 与本轮相关的 Rust 核心回归 19 项通过；另有路径边界测试。旧有依赖个人 Java、配置、游戏目录或外部服务的测试不作为离线回归运行。
- 本地 HTTP 模拟服务覆盖完整安装：503 重试、缺失 server 元数据、classifier-only 库、native 排除与 Zip Slip 拒绝、资源去重/旧布局、共享缓存复用、自定义实例名与重新扫描、已有目录保护、版本 JSON 篡改、文件大小/SHA-1 错误、失败不发布实例、关闭 UI 通道后继续完成安装。
- `scripts/check-components.mjs` 扩展了真实 Pinia store 的边界回归：下载阶段全部完成但发布失败、终态后的迟到 IPC、并行任务、速度清零、IPC ID 隔离、导出防重复。
- `pnpm ui:build` 通过，包含 UI/语言/组件/页面切换契约、Vue 类型检查和生产构建。
- 官方服务验证通过：manifest v2 的真实元数据与资源索引能够为 `rd-132211`、`b1.7.3`、`1.2.5`、`1.16.5`、`1.21.1` 和本次官方最新正式版 `26.3` 生成安装计划。这里没有下载这六个版本的全部文件。
- 官方 `1.2.5` 的自定义命名完整安装通过：实际下载并校验客户端、支持库、native 与资源，完成旧资源映射，再通过实例扫描确认实例名、原版版本号与 JAR。
- 官方 `1.2.5` 客户端及服务端导出通过，包含服务端启动脚本；未执行这些脚本。
- `cargo clippy -p pcl-proto --lib` 执行完成，仓库及 Terracotta 仍有旧代码告警，未将此次迁移扩展成全仓 lint 清理。

复现离线检查：

```sh
cargo test -p pcl-proto --lib core:: -- \
  --skip core::api_client::tests::forge_test \
  --skip core::api_client::tests::mc_manifest \
  --skip core::java:: \
  --skip core::launcher::game_launch_test \
  --skip core::platform::read_header_test
cargo test -p pcl-proto --lib util::file::path_tests
pnpm ui:build
git diff --check
```

实网检查显式运行，测试使用独立临时目录，结束时删除其测试数据：

```sh
cargo test -p pcl-proto --lib official_metadata_across_generations -- --ignored --nocapture
cargo test -p pcl-proto --lib official_vanilla_install_with_custom_name -- --ignored --nocapture
cargo test -p pcl-proto --lib official_client_and_server_export -- --ignored --nocapture
```

## 当前边界与下一批

本批没有验证真实游戏进程启动，也没有完成现代 `arguments.game/jvm`、旧 `minecraftArguments`、版本继承、Java 版本/架构选择和实例隔离的整体迁移。旧版 x64 native 在 ARM 主机上能下载和解压，并不代表能用 ARM Java 启动。BMCLAPI 的地址转换有离线测试，但没有进行 BMCLAPI 真实服务安装验收；下载源自动竞速/降级、分段下载、断点续传、用户取消及应用崩溃后的安装恢复尚未实现。新安装拒绝覆盖已有目录，原地修复另做明确入口。

后续按依赖推进：

1. **启动链路**：统一解析继承后的版本、Mojang 参数与占位符、所选 Java 的版本/架构、日志与进程状态；在真实桌面环境验收新旧原版启动，再做缺失依赖修复。
2. **加载器安装**：先 Fabric / LegacyFabric 的版本继承与库补全，再 Forge / NeoForge 的 installer/processor；OptiFine、其他加载器和 API 附加项分别按 CE 的兼容与互斥规则接入。当前 UI 仍明确阻止尚未实现的加载器安装。
3. **配置和账户**：将前端偏好映射到 Rust 配置、处理持久化和重置，分离账户展示数据与凭据，再接微软 OAuth/令牌刷新及外置登录。
4. **资源与整合包**：将 Modrinth/其他来源的检索接到真实文件安装、依赖解析、更新及收藏批量操作，之后接整合包导入/导出和存档操作。

Windows/Linux 真实桌面行为、macOS WebView 安装交互、Java 扫描、账户认证和联机仍需各自验收，不能用本批 Rust 服务测试代替。
