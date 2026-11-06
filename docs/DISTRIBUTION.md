# 分发 pcl-core

目标：`pcl-core` 可以独立给第三方使用，别人能拿它做自己的启动器。

## 当前策略：Git 分发

暂不发布 crates.io，使用者按提交固定：

```toml
[dependencies]
pcl-core = { git = "https://github.com/PCL-Community/PCL.Proto", rev = "<commit>" }

# 需要 CE 兼容的启动补丁 / Forge 安装桥接时
pcl-assets = { git = "https://github.com/PCL-Community/PCL.Proto", rev = "<commit>" }

# 需要联机能力时
pcl-core = { git = "https://github.com/PCL-Community/PCL.Proto", rev = "<commit>", features = ["link"] }
```

建议用 `rev` 或 `tag` 而不是分支，避免上游变动影响下游构建。

## 已验证

| 检查 | 结果 |
| --- | --- |
| 独立消费者以 path 依赖 `crates/pcl-core` + `crates/pcl-assets` | 编译通过（默认 feature） |
| 消费者以 `git = "..."` 依赖 | 编译通过；Cargo 会自动拉取 `crates/terracotta` 子模块 |
| `features = ["link"]` | 编译通过（子模块 `crates/terracotta` 拉取并参与编译） |
| `cargo publish --dry-run -p pcl-assets` | 通过 |
| 核心不读写配置、不含桌面资源、无外部路径 `include_*!` | 是 |

工作区继承（`version.workspace`、`serde = { workspace = true }`）在 Git 依赖下可以正确解析。