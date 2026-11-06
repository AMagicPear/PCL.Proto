# 代码组织与维护边界

更新：2026-11-01。本文描述当前实现的职责划分；功能对齐记录见 [CORE_ALIGNMENT.md](CORE_ALIGNMENT.md)，设置规划见 [SETTINGS_DESIGN.md](SETTINGS_DESIGN.md)。规划中的能力不等于已经实现。

## 前端

仓库根目录的 `package.json`、Vite 和 TypeScript 配置负责网站与 Tauri 前端；
`packages/components/` 保存组件 npm 包的入口、样式、包描述和独立构建配置，
直接复用 `src/components/widget/` 中的控件。根目录 `Cargo.toml` 管理 Rust 工作区：
`crates/pcl-core/` 是启动器核心，`src-tauri/` 是桌面应用，`src-terracotta/` 是独立维护的 Git 子模块；
不要把子模块文件复制到组件包。组件包运行 `bun run components:build` 构建，或执行
`npm pack ./packages/components` 构建并打包。

| 位置                     | 负责的内容                                                   |
| ------------------------ | ------------------------------------------------------------ |
| `src/views/`             | 页面布局、路由参数、该页面的交互与展示状态                   |
| `src/components/widget/` | 复用控件、列表项和领域展示组件                               |
| `src/composables/`       | 组件生命周期、响应式编辑状态、页面切换时的失效保护           |
| `src/stores/`            | 跨页面共享的账户、Java、启动、下载任务等状态                 |
| `src/api/`               | 领域设置、IPC 参数、资源服务请求和来源归一化                 |
| `src/types/`             | 共享数据模型；资源项目、版本、来源与种类统一定义在 `comp.ts` |
| `src/util/`              | 公共机制，如外链、平台、菜单和设置持久化队列                 |

资源数据的依赖方向：

```text
types/comp.ts
  ↑ Modrinth / CurseForge 客户端、筛选规则、收藏、组件
  ↑ api/compProjects.ts（来源分发、双来源搜索与合并）
  ↑ 页面与下载流程
```

两个来源都输出相同的资源模型；CurseForge 不依赖 Modrinth 客户端的类型定义。来源专有的响应类型、请求参数和校验保留在各自客户端。`CompType` 是资源种类的单一定义。

### 设置持久化

启动参数、内存、窗口、下载和实例启动设置各自创建一个 `createSettingsPersistence` 实例。公共实现位于 `src/util/settingsPersistence.ts`，统一处理同一作用域的串行保存、未保存草稿、错误和旧请求的完成顺序。

- 全局作用域是 `null`，实例作用域是实例目录；不同作用域独立排队。
- `read(directory?)` 等待当前保存队列，返回 `{ settings, error, dirty }`；有草稿时优先保留草稿。
- `stage(settings, directory?)` 只暂存；启动参数在离开输入框、切换实例或卸载时使用 `save(directory?)` 提交。
- `write(settings, directory?)` 暂存并提交；失败保留草稿，可再次保存。
- 输入与读取结果都复制成独立快照；旧保存结果不能清除新草稿或覆盖新错误状态。

设置领域负责各自的默认值、IPC 负载和业务校验。组件负责就绪状态、输入转换和本组件的错误展示。实例启动设置的保存队列属于领域模块，页面重新挂载后仍复用同一队列。

## Rust 后端

Rust 侧分成两个 crate：`crates/pcl-core/` 承载全部领域逻辑且不依赖 Tauri；
`src-tauri/` 只是 Tauri 宿主，负责 IPC、窗口、资源协议和平台打包。
应用只依赖 `pcl-core`，由核心库按需再引入 Terracotta（`link` feature）；
第三方启动器可以复用 `pcl-core` 并注入自己的凭据与界面。

| crate / 位置            | 负责的内容                                                             |
| ----------------------- | ---------------------------------------------------------------------- |
| `pcl-core/src/app/`     | 凭据注入、可见性判定、启动器自带的只读资源定位                         |
| `pcl-core/src/runtime.rs` | `RuntimePaths`：宿主提供的配置目录与 Java 运行时目录                  |
| `pcl-core/src/io/`      | 下载器、文件操作、HTTP 客户端、代理策略                                |
| `pcl-core/src/minecraft/` | 版本、实例、安装、Java、启动、账户、导出与整合包                     |
| `pcl-core/src/link/`    | 服务器列表 ping；启用 `link` feature 时提供 Terracotta 桥接            |
| `pcl-core/src/events.rs` | `EventSink`：后台任务回传进度的普通回调，替代 Tauri 的 `Channel`      |
| `src-tauri/src/config/` | `ConfigManager`、`AppState`、档案与登录会话持久化、设置到领域类型的折算 |
| `src-tauri/src/login/`  | 交互式登录流程（设备码、档案选择、取消与状态轮询）                     |
| `src-tauri/src/commands/` | 130 个 `#[tauri::command]` 适配层，命令名与参数保持不变             |
| `src-tauri/src/`        | 应用初始化、窗口动作、自定义 URI 协议、macOS 资源与凭据读取            |

配置系统属于宿主应用：`config.json` 的路径、读写、单例与档案持久化都在 `src-tauri`；
`pcl-core` 只接收显式输入（`AppState` 折算后的启动请求、`JavaState`、`RuntimePaths`、
下载设置等），不读配置文件、不持全局状态。

凭据由宿主注入：`src-tauri/src/secrets.rs` 按"编译期烧录 > 环境变量"读取
`CURSEFORGE_API_KEY` 和 `MS_CLIENT_ID`，启动时调用 `pcl_core::app::secrets::install`；
核心库本身不含任何内置密钥，未注入时按"未配置"处理。

`src-tauri/src/commands/mod.rs` 只声明命令模块。`lib.rs` 直接注册各模块的命令；模块调整不改变前端 IPC 名称或参数。

| 命令模块                | 职责                                                     |
| ----------------------- | -------------------------------------------------------- |
| `launch.rs`             | 启动控制、取消、进度转发和启动脚本导出；共用启动计划构建 |
| `launch_settings.rs`    | 全局/实例启动参数与实例启动附加设置                      |
| `memory.rs`             | 内存设置和预览                                           |
| `java.rs`               | Java 列表、偏好、探测、安装与安装取消                    |
| `instances.rs`          | 实例选择、说明、重命名、隔离与实例目录操作               |
| `instance_resources.rs` | 实例资源目录、文件列表、导入、启禁、删除与修复           |
| `repositories.rs`       | Minecraft 文件夹管理和文件夹内的实例查询                 |
| `versions.rs`           | 游戏版本与加载器元数据查询                               |
| `network.rs`            | 代理设置                                                 |
| `account.rs`            | 返回不含凭据的账户投影                                   |
| `skin.rs`               | 皮肤解析、缓存、另存与正版皮肤上传                       |
| `config.rs`             | 窗口、可见性、版本说明和日志行数设置                     |
| `download.rs`           | 下载任务、下载资源配额与下载设置                         |
| `sources.rs`            | Modrinth / CurseForge 元数据请求与可用性                 |
| `installer.rs`          | 原版安装、Forge 系安装器、整合包导入                     |
| `export.rs`             | 客户端/服务端导出、加载器文件与资源下载                  |
| `session.rs`            | 运行中游戏会话的查询、停止与日志清理                     |
| `system.rs`             | 外链、系统字体、今日人品和服务器 ping                    |
| `link.rs`               | Terracotta 联机桥接                                      |

命令层只负责参数适配与 IPC 通道：进度用 `channel_sink` 把 `Channel` 接到核心库的 `EventSink`，状态通过 `ConfigManager` 取得。Minecraft、Java、下载和文件操作的领域实现位于 `pcl-core` 的 `minecraft/`、`io/` 等模块。`ConfigManager` 负责事务更新与落盘。目录边界、符号链接检查、配置保存失败处理和重命名回滚不能在整理时省略。

## 维护约定与验证

`scripts/` 保存应用共用的图标生成与 CE 资源同步工具；包专属的构建脚本保留在所属包中。前端回归检查统一放在 `tests/ui/*.test.mjs`，由 `bun run check:ui` 使用 Bun 测试运行器逐文件执行；新增测试无需修改命令列表。临时界面对照入口不随应用保留。

新增功能先找所属领域的现有入口。公共规则只保留一个实现，调用方直接 import 所属模块；不要为转调一次函数而增加页面包装、兼容转导出或新的 service 层。页面确有独有状态、校验、错误处理或交互时再保留本地函数。

本次整理的设置回归位于 `tests/ui/settings-persistence.test.mjs`，覆盖串行与并行作用域、快照隔离、失败重试、过期完成、读取期间编辑、真实领域 IPC 负载和实例编辑器重新挂载。该检查已接入 `bun run check:ui`。

常规验证：`bun run ui:build`、`cargo check --workspace`、`cargo test -p pcl-core --lib`、`cargo test -p pcl-proto --lib`。离线检查与构建不代表已完成真实游戏启动或 Windows/Linux 桌面验收。
