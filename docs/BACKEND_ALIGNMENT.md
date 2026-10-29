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
| `src-tauri/src/core/downloader.rs`、`downloader/` | 共享 HTTP 客户端、全局连接与带宽配额、Range 分段与会话内续传、下载源回退、校验、临时文件发布与统计 |
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

## 第二批：下载策略与活动连接

旧页面的活动连接写死为 `—`，Rust 没有提供活动请求统计。旧下载器每个安装任务同时处理 8 个文件，每个文件只有一个请求，出错后重新下载整个文件；多个安装任务也没有共享连接上限。设置页的下载选项此前只保存在浏览器中。

本轮参考 CE 的 `AdaptiveRangeDownloader.cs`、`DownloadResourceManager.cs`、`FileDownloader.cs`、`ModDownload.cs` 和 `ModSetup.cs` 迁移原版安装与客户端/服务端导出的共享下载器：

| 行为 | 当前实现 |
| --- | --- |
| 连接配额 | 所有安装/导出共享，默认 64，范围 1–256；单文件默认 8，范围 1–8。降低上限时允许已有请求完成，后续请求遵守新值 |
| 小文件 | 小于 4 MiB 直接顺序下载，避免 Range 探测往返；复用连接池 |
| 大文件 | 先用一字节 Range 探测；按约 8 MiB 划分，初始最多 1024 段，失败拆分后最多 2048 段；每段使用独立文件句柄与偏移 |
| 中断与慢段 | 保留当前会话内已写入部分，仅续传剩余区间；剩余至少 2 MiB 时可拆分。运行至少 8 秒、还有至少 1 MiB 且明显慢于同伴的分段才恢复；限速时关闭慢段判断 |
| Range 降级 | 忽略 Range、416、Content-Range/长度/编码不可靠时取消其他分段，丢弃临时文件并回退顺序下载；最终仍验证完整文件 SHA-1 |
| 下载源 | 官方与 BMCLAPI 按偏好排序；库同时包含 `/maven` 和 `/libraries`。永久 4xx 跳过该来源，网络、流中断和校验错误可重试/换源；第三方来源不随意重写 |
| 自动源选择 | 官方 manifest 成功用时低于 4 秒时，自动文件源优先官方，否则优先镜像。版本列表单独读取来源设置；Auto 官方请求超时为 5 秒，随后回退镜像 |
| HTTP | Auto 下小文件优先协商 HTTP/2，大文件/Range 使用 HTTP/1.1；显式 HTTP/2 仍允许 HTTP/1.1 回退，协商失败也可重试 HTTP/1.1 |
| 限速 | 共享令牌桶，128 KiB 突发容量；设置页滑块按 CE 的分段公式转换为字节/秒，可实时调整 |
| 页面统计 | 每 300 ms 查询全局活动请求数/上限、实际接收速度；包括重试流量，不把缓存命中计为网络流量。离开页面停止轮询，迟到响应不更新已卸载组件 |
| 磁盘 | 64 KiB 缓冲写入，完整校验后发布；可重新下载的缓存不逐个执行 `fsync` |

设置页的下载源、版本列表源、连接数、单文件连接数、限速和 HTTP 协议均已接入 Rust。安装、导出和获取版本列表时也携带已保存的设置，首次下载不依赖先打开设置页。活动连接表示正在执行的 HTTP 请求/HTTP2 流，并不等于系统 TCP socket 数。

新增验证：

- Rust 核心离线回归 29 项通过，另有 1 项路径边界测试。覆盖全局/单文件上限、实时调整和取消释放、共享限速、分段拼接及断流续传、Range 探测及实际分段响应异常的顺序降级、校验失败保留原文件、临时文件清理、小文件不探测与缓存零流量。
- 前端检查覆盖 CE 滑块边界、损坏配置回退、持久化设置传入安装/导出 IPC；`pnpm ui:build` 通过。
- Chrome 使用真实 Vue 面板与模拟 Tauri 统计，验证 `23 / 64`、速度显示、空闲归零、失败清除旧数据、恢复以及卸载后停止请求。这不替代真实 Tauri WebView 验收。
- 实网下载官方 `1.21.1` 客户端通过：文件 26,836,906 字节，实际接收 26,836,907 字节（含一字节 Range 探测），下载/校验用时 17.163 秒。
- 本地按请求限速服务的单次对照：同一份 32 MiB 文件、同一配置下，单连接顺序下载 2.777 秒，最多 8 连接的 Range 下载 1.750 秒；两份内容及 SHA-1 一致，Range 额外接收一字节。文件实际分成 4 段；使用 Rust debug 构建，计时包含校验和磁盘操作。此结果仅说明单请求受限场景的收益，**不是 CE 实际运行对比，也不是实网加速承诺**。
- `cargo clippy -p pcl-proto --lib` 执行通过；已有全仓告警仍单独保留。

额外显式检查：

```sh
cargo test -p pcl-proto --lib controlled_range_benchmark -- --ignored --nocapture
cargo test -p pcl-proto --lib official_large_client_range_smoke -- --ignored --nocapture
```

与 CE 的剩余差异：版本列表目前按偏好顺序超时/失败回退，尚未复刻 CE 的延时并行竞速；文件切换来源或整个传输重试会重新下载该文件，不持久化跨来源分段。没有新增 UI 取消入口、跨应用重启断点恢复或任意未知大小 URL 的通用下载入口。

## 第三批：Java 管理、选择与多平台运行时

参考 CE 的 `ModJava.cs`、`ModLaunch.cs`、`PageSetupJava.xaml.cs`、`PageInstanceSetup.xaml.cs`，以及 `PCL.Core/Minecraft/Java` 下的管理器、扫描器、解析器和用户偏好。保留 CE 的选择契约，将 Windows 路径与架构规则扩展至 macOS、Linux。

本次 26.3 故障的直接原因是实际执行了 Java 21（最高 class 65），游戏要求 Java 25（class 69）。本机旧配置曾将 `/usr/bin/java` 缓存为 25，但实际探测为 Zulu 21.0.7；新逻辑只迁移旧选择路径，版本与架构重新从可执行文件读取。

| 链路 | 当前行为 |
| --- | --- |
| 探测 | 执行 `-XshowSettings:properties -version`，旧 Java 回退 `-version`；校验真实版本、Java Home、架构和供应商，解析 Java 6/8 更新号及现代 EA 版本。处理系统 stub、wrapper、符号链接、Java 8 嵌套 JRE；超时 5 秒，输出有界且同步排空两个管道 |
| 扫描 | 环境变量、PATH、系统 JVM 根、SDKMAN/asdf/jabba/mise、Minecraft 与启动器 runtime；Windows 追加注册表和 Store runtime，macOS 追加 JDK bundles、Applications、Homebrew。关键词遍历有深度限制并防链接循环；自动扫描间隔至少 13 秒，手动刷新强制重扫 |
| 状态 | 统一 Rust 配置保存全局路径、运行时启用/禁用和实例偏好；扫描保留禁用状态与并发导入，失效项移除。配置写入临时文件后替换，序列化失败保留原文件；旧实例 localStorage 偏好只在没有新设置时迁移 |
| 需求 | 原版/快照日期、OptiFine、Forge（含 Java 7 和 8u321 上限）、Fabric、LiteLoader、Cleanroom、LabyMod；读取 JSON `javaVersion`、非连续 `compatibleJavaMajors` 集合与 JAR `version.json`。仅为 Java 需求解析父版本元数据，并检查继承循环 |
| 选择 | 实例指定/启动器目录内相对路径 → 全局指定 → 自动候选；每次选择重新探测实际 executable。自动候选按最低可接受 major、原生架构、JDK、供应商、最新补丁排序，禁用、失效或不能运行的候选不会启动 |
| 手动覆盖 | 与当前 CE 一致：指定版本越界时警告后仍使用，忽略兼容性只跳过警告。CE 当前 `AutoSelect` 在启动选择时仍继续检查全局指定；实例 Auto 不会绕过全局手动覆盖 |
| 补全 | 缺失时重扫；用户确认后按平台/架构及 Mojang component 下载，再重新选择。复用共享下载源、连接/带宽配额、大小/SHA-1 校验；独立 staging、失败/取消清理、Unix 执行权限与安全链接处理。旧 Java 和受限 Java 8 更新范围要求手动安装 |
| 界面 | 全局自动/指定、导入、刷新、详情、打开目录、启用/禁用；实例全局/自动/相对/指定、忽略兼容性；32 位内存警告读取后端 Java 信息。启动前提供下载进度和取消，拒绝异步期间切换到其他实例后继续启动 |

平台处理：Windows x86/x64/ARM64、macOS Intel/Apple Silicon、Linux x86/x64/ARM64 的本地 Java 均可识别，架构兼容性按实际主机 CPU 判断（包含 Rosetta/Windows 模拟下的启动器）。可运行性最终由实际执行验证。Mojang 当前没有 Linux ARM64 运行时，也未必提供每个平台的所有 Java major；这种情况明确提示手动安装，不使用其他架构冒充原生运行时。Linux ARM64 可导入本地 ARM64 Java。选择 ARM Java 并不代表实例已有的 native 库也已经匹配 ARM，二者联动仍属于后续启动链路工作。

Java 验证：

- 本轮 Rust 核心离线回归 47 项通过（其中 Java 18 项），另有配置回归 2 项、路径边界回归 1 项通过。Java 覆盖需求规则、major 集合、排序、缓存版本失真、手动越界警告、禁用/失效回退、别名去重、Java 6 回退、Java 8 JDK、相对路径与 manifest 链接边界、继承循环、扫描目录布局、探测大输出/超时，以及模拟 HTTP 完整安装、错误散列和取消清理。`pnpm ui:build` 通过。
- 配置回归覆盖旧字段迁移、不覆盖新偏好、保存失败保留旧文件；前端真实 Pinia store 的模拟 IPC 回归覆盖迁移、持久化、保存失败、迟到请求隔离、相对路径和禁用候选。
- macOS ARM64 本机只读扫描通过，识别本地 Java 8、17、21，以及用户 Applications 中 Rider 自带的 Java 25.0.2。官方 ARM64 Java 25.0.1 已在独立临时目录完整下载、校验、执行并清理，报告原生架构兼容；没有改写用户启动器配置。
- Chrome 使用真实 Vue 组件与模拟 Tauri IPC，检查全局选择、启用/禁用、实例选项和相对路径保存；这不等于原生 Tauri 验收。
- Windows/Linux/macOS 构建矩阵已加入 Java 和配置测试；尚未执行远端 CI。本机 Windows 交叉检查在下载目标依赖阶段受阻，没有取得 Windows 编译结果。

显式只读/实网检查：

```sh
cargo test -p pcl-proto --lib setup::java_config_tests
cargo test -p pcl-proto --lib native_java_discovery_smoke -- --ignored --nocapture
cargo test -p pcl-proto --lib official_native_java25_install_smoke -- --ignored --nocapture
```

## 当前边界与下一批

本批没有验证真实游戏进程启动，也没有完成现代 `arguments.game/jvm`、旧 `minecraftArguments`、通用版本继承、native 与所选 Java 架构联动和实例隔离的整体迁移。Java 管理与选择见第三批。旧版 x64 native 在 ARM 主机上能下载和解压，并不代表能用 ARM Java 启动。BMCLAPI 的地址转换有离线测试，但没有进行 BMCLAPI 真实服务安装验收；分段下载、会话内续传和下载源降级见第二批；版本列表竞速、Minecraft 安装的 UI 用户取消及应用崩溃后的安装恢复尚未实现。新安装拒绝覆盖已有目录，原地修复另做明确入口。

后续按依赖推进：

1. **启动链路**：统一解析继承后的版本、Mojang 参数与占位符、所选 Java 的版本/架构、日志与进程状态；在真实桌面环境验收新旧原版启动，再做缺失依赖修复。
2. **加载器安装**：先 Fabric / LegacyFabric 的版本继承与库补全，再 Forge / NeoForge 的 installer/processor；OptiFine、其他加载器和 API 附加项分别按 CE 的兼容与互斥规则接入。当前 UI 仍明确阻止尚未实现的加载器安装。
3. **配置和账户**：将前端偏好映射到 Rust 配置、处理持久化和重置，分离账户展示数据与凭据，再接微软 OAuth/令牌刷新及外置登录。
4. **资源与整合包**：将 Modrinth/其他来源的检索接到真实文件安装、依赖解析、更新及收藏批量操作，之后接整合包导入/导出和存档操作。

Windows/Linux 真实桌面行为与 Java 扫描、macOS WebView 安装交互、账户认证和联机仍需各自验收，不能用本批 Rust 服务测试代替。
