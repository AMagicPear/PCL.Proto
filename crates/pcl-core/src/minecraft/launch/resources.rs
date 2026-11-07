//! 启动兼容资源；默认按功能需要准备，也可由宿主打包后传入替代路径。

pub const JAVA_WRAPPER_NAME: &str = "java-wrapper.jar";
pub const LEGACY_FIX_NAME: &str = "legacyfix.jar";
pub const LWJGL_UNSAFE_AGENT_NAME: &str = "lwjgl-unsafe-agent.jar";
pub const LOG4J2_DEBUG_NAME: &str = "log4j2-debug.xml";
pub const LOG4J2_LEGACY_DEBUG_NAME: &str = "log4j2-legacy-debug.xml";

pub const JAVA_WRAPPER: &[u8] = include_bytes!("../../../resources/launch/java-wrapper.jar");
pub const LEGACY_FIX: &[u8] = include_bytes!("../../../resources/launch/legacyfix.jar");
pub const LWJGL_UNSAFE_AGENT: &[u8] =
    include_bytes!("../../../resources/launch/lwjgl-unsafe-agent.jar");
pub const LOG4J2_DEBUG: &[u8] = include_bytes!("../../../resources/launch/log4j2-debug.xml");
pub const LOG4J2_LEGACY_DEBUG: &[u8] =
    include_bytes!("../../../resources/launch/log4j2-legacy-debug.xml");
pub const MACOS_ARM_LIBRARIES: &str =
    include_str!("../../../resources/launch/macos-arm64-libraries.json");
