# pcl-assets

PCL.Proto 的可选启动器资源，供希望获得与原启动器一致行为的宿主使用。

| 文件 | 用途 |
| --- | --- |
| `java-wrapper.jar` | Java 启动包装器（Windows 代码页兜底） |
| `legacyfix.jar` | 1.6 以下版本的启动修复 |
| `lwjgl-unsafe-agent.jar` | LWJGL 3.4.1 的 unsafe 内存代理 |
| `log4j2-debug.xml` / `log4j2-legacy-debug.xml` | 调试日志配置 |
| `forge-installer.jar` | CE 的无头 Forge / NeoForge 安装桥接，**安装 Forge 必需** |
| `macos-arm64-libraries.json` | macOS ARM64 的原生库替换映射（来自 HMCL） |

`pcl-core` 不依赖这个 crate；把资源落盘后把路径传给核心即可：

```rust
let dir = config_dir.join("resources");
pcl_assets::materialize(&dir)?;
let forge_bridge = dir.join(pcl_assets::FORGE_INSTALLER_NAME);
```

资源文件来自 [PCL2-CE](https://github.com/PCL-Community/PCL2-CE)（Apache-2.0）与 HMCL。
