# Launcher resources

Copied from [PCL2-CE](https://github.com/PCL-Community/PCL2-CE) `Plain Craft Launcher 2/Resources/`
(Apache License 2.0) so the instance settings page can launch games the same way:

- `java-wrapper.jar` — Java Launch Wrapper (`CheckAdvanceDisableJLW`)
- `legacyfix.jar` — LegacyFix for versions before 1.6 (`CheckAdvanceDisableLF`)
- `lwjgl-unsafe-agent.jar` — LWJGL 3.4.1 unsafe-memory agent (`CheckAdvanceDisableLwjglUnsafeAgent`)
- `log4j2-debug.xml`, `log4j2-legacy-debug.xml` — debug logging configs (`CheckUseDebugLog4j2Config`)

They are embedded in the binary with `include_bytes!` and extracted to
`<config dir>/resources` on first use. The renderer agent (mesa-loader) is downloaded from the
Tencent maven mirror at launch time instead of being bundled.
