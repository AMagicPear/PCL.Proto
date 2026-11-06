# 发布 pcl-core

目标：`pcl-core` 可以独立发布，第三方拿来做自己的启动器。

## 已经就绪

- 不依赖 Tauri、不读写配置文件、不携带桌面资源；所有宿主相关输入都是显式参数
  （`RuntimePaths`、`JavaState`、`LaunchHelpers`、`EventSink`、`Secrets`）。
- 没有外部路径的 `include_bytes!` / `include_str!`；资源在独立的 `pcl-assets`。
- `cargo publish --dry-run -p pcl-assets` 通过。
- `crates/pcl-core/README.md` 说明了定位、用法和不负责的部分。
- 公开类型可构造：`LaunchRequest`、`LoaderInstall`、`LoaderFamily`、`JavaState`、
  `JavaRoots`、`RuntimePaths`、`Secrets` 都有公开字段。

## 阻塞项

### 1. `link` feature 依赖未发布的 Terracotta

`cargo package -p pcl-core` 在解析依赖时失败：

```text
failed to select a version for the requirement `terracotta = "^2.5.0-pcl.proto"`
candidate versions found which didn't match: 0.4.2, 0.4.1, 0.4.0, ...
```

两个事实：

- `crates/terracotta` 是 Git 子模块（`Terracotta-lib` 的 fork，AGPL-3.0），不在 crates.io；
- crates.io 上的 `terracotta` 这个名字**已被别的项目占用**（0.4.x），我们的 fork 不能沿用。

可选路线：

| 方案 | 做法 | 结果 |
| --- | --- | --- |
| A | 把 fork 以新名字（如 `pcl-terracotta`）发布到 crates.io | `pcl-core` 能带 `link` 发布，但需接受 AGPL 分发 |
| B | 把联机桥接拆成 `crates/pcl-link`（依赖 `pcl-core` + terracotta） | **`pcl-core` 立即可以发布**；联机能力作为独立 crate 稍后发布 |
| C | 只支持 git 分发 | 需要把 terracotta 从子模块改为随仓库提交，否则使用者拉不到 |

推荐 **B**：核心保持单机能力、零可选依赖；联机是叠加能力，和"桌面应用资源"一样不该绑在核心上。
这也和当前 `pcl-assets` 的处理方式一致。

### 2. 许可证标识

`license = "PCL-License"` 不是 SPDX 标识。crates.io 会校验 `license` 字段；
发布前需要二选一：

- 使用 `license-file = "LICENSE"`（把根目录许可证复制进每个待发布 crate）；
- 或改用标准 SPDX 标识（需要作者决定，属于法务选择）。

`LICENSE` 本身是自定义的"PCL 使用指南"，不是 OSI 许可证；发布前请确认是否接受第三方
按此条款使用。

## 发布顺序

1. 解决 Terracotta（方案 B 则跳过）。
2. 处理许可证字段。
3. `cargo publish -p pcl-assets`
4. `cargo publish -p pcl-core`
5. （方案 B）`cargo publish -p pcl-link`

发布前验证：`cargo publish --dry-run -p <crate>`、`cargo test -p <crate> --lib`。
