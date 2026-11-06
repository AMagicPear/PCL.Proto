pub mod http;
pub mod proxy;

use std::sync::OnceLock;

static USER_AGENT: OnceLock<String> = OnceLock::new();

/// 宿主应用启动时设置自己的 UA；未设置时使用中性的引擎标识。
pub fn set_user_agent(value: impl Into<String>) {
    let _ = USER_AGENT.set(value.into());
}

pub fn user_agent() -> &'static str {
    USER_AGENT
        .get()
        .map(String::as_str)
        .unwrap_or(concat!("pcl-core/", env!("CARGO_PKG_VERSION")))
}
