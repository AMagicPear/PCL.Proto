//! 启动器外呼凭据（CurseForge Client ID、微软 OAuth Client ID）。
//!
//! 核心库不自己读取密钥：宿主应用在启动早期用 [`install`] 注入，
//! 这样第三方启动器可以复用核心逻辑并换成自己的密钥。
//! 未注入时所有凭据视为未配置，相关功能按不可用处理。

use std::sync::RwLock;

/// 宿主注入的外呼凭据。
#[derive(Clone, Debug, Default)]
pub struct Secrets {
    pub curseforge_api_key: Option<String>,
    pub ms_oauth_client_id: Option<String>,
}

static SECRETS: RwLock<Secrets> = RwLock::new(Secrets {
    curseforge_api_key: None,
    ms_oauth_client_id: None,
});

/// 注入凭据；后注入的值整体覆盖先前的配置。
pub fn install(secrets: Secrets) {
    *SECRETS
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = secrets;
}

/// 空白值按未配置处理。
fn configured(value: Option<&str>) -> Option<String> {
    let value = value?.trim();
    (!value.is_empty()).then(|| value.to_owned())
}

/// CurseForge API 的 Client ID。
pub fn curseforge_api_key() -> Option<String> {
    let secrets = SECRETS
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    configured(secrets.curseforge_api_key.as_deref())
}

/// 微软 OAuth 的 Client ID。
///
/// 没有内置默认值：旧版公开客户端 `00000000402b5328` 已被微软设备码端点拒绝
/// （`AADSTS700016`），回退到它只会把配置缺失伪装成登录失败。
pub fn ms_oauth_client_id() -> Option<String> {
    let secrets = SECRETS
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    configured(secrets.ms_oauth_client_id.as_deref())
}

/// CurseForge 是否可用，供界面隐藏或禁用相关选项（对照 CE 的 SupportCurseForge）。
pub fn curseforge_available() -> bool {
    curseforge_api_key().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_and_missing_keys_are_treated_as_unconfigured() {
        assert_eq!(configured(None), None);
        assert_eq!(configured(Some("   ")), None);
        assert_eq!(configured(Some("  key-id  ")).as_deref(), Some("key-id"));
    }
}
