//! 读取本应用的 CurseForge / 微软 OAuth 凭据并注入 `pcl-core`。
//!
//! 核心库不决定密钥来源：发布构建在编译期把值烧进二进制（对照 CE 的 Source Generator），
//! 开发构建回退到运行环境变量；第三方启动器可以完全换一套读取方式。

use pcl_core::app::secrets::{self, Secrets};

/// 空白值按未配置处理。
fn usable(value: Option<String>) -> Option<String> {
    let value = value?.trim().to_owned();
    (!value.is_empty()).then_some(value)
}

fn compile_time(value: Option<&'static str>) -> Option<String> {
    usable(value.map(str::to_owned))
}

fn from_env(name: &str) -> Option<String> {
    usable(std::env::var(name).ok())
}

/// 在核心库初始化前调用一次。
pub fn install() {
    secrets::install(Secrets {
        curseforge_api_key: compile_time(option_env!("CURSEFORGE_API_KEY"))
            .or_else(|| from_env("CURSEFORGE_API_KEY"))
            .or_else(|| from_env("PCL_CURSEFORGE_API_KEY")),
        ms_oauth_client_id: compile_time(option_env!("MS_CLIENT_ID"))
            .or_else(|| from_env("MS_CLIENT_ID"))
            .or_else(|| from_env("PCL_MS_CLIENT_ID")),
    });
}
