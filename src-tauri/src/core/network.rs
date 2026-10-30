//! Process-wide HTTP proxy policy shared by downloads, metadata and game launch.
use std::sync::{
    LazyLock, RwLock,
    atomic::{AtomicU64, Ordering},
};

use crate::setup::constants::USER_AGENT;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyMode {
    #[default]
    Off,
    System,
    Custom,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProxySettings {
    pub mode: ProxyMode,
    /// Custom proxy URL, possibly carrying credentials (`http://user:pass@host:port`).
    pub address: String,
}

impl ProxySettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.address.len() > 2048 || self.address.contains('\0') {
            return Err("Invalid proxy address".into());
        }
        if self.mode != ProxyMode::Custom {
            return Ok(());
        }
        let url = reqwest::Url::parse(self.address.trim())
            .map_err(|_| "Invalid proxy address; expected http://host:port".to_string())?;
        if !matches!(url.scheme(), "http" | "https")
            || url.host_str().is_none()
            || url.port().is_none()
        {
            return Err("Invalid proxy address; expected http://host:port".into());
        }
        Ok(())
    }

    /// Host and port passed to the game JVM (CE only forwards the custom proxy).
    pub fn game_endpoint(&self) -> Option<(String, u16)> {
        if self.mode != ProxyMode::Custom {
            return None;
        }
        let url = reqwest::Url::parse(self.address.trim()).ok()?;
        Some((url.host_str()?.to_owned(), url.port()?))
    }
}

static SETTINGS: LazyLock<RwLock<ProxySettings>> =
    LazyLock::new(|| RwLock::new(ProxySettings::default()));
static GENERATION: AtomicU64 = AtomicU64::new(0);

pub fn current() -> ProxySettings {
    SETTINGS.read().unwrap().clone()
}

/// Bumped on every applied change so lazily built clients can rebuild.
pub fn generation() -> u64 {
    GENERATION.load(Ordering::Acquire)
}

pub fn apply(settings: &ProxySettings) -> Result<(), String> {
    settings.validate()?;
    *SETTINGS.write().unwrap() = settings.clone();
    GENERATION.fetch_add(1, Ordering::AcqRel);
    Ok(())
}

/// Client builder that honours the current proxy policy.
pub fn builder() -> reqwest::ClientBuilder {
    builder_for(&current())
}

fn builder_for(settings: &ProxySettings) -> reqwest::ClientBuilder {
    let builder = reqwest::Client::builder().user_agent(USER_AGENT);
    match settings.mode {
        ProxyMode::Off => builder.no_proxy(),
        // reqwest already follows the system proxy configuration by default.
        ProxyMode::System => builder,
        ProxyMode::Custom => match reqwest::Proxy::all(settings.address.trim()) {
            Ok(proxy) => builder.proxy(proxy),
            // Validation rejects malformed addresses, so this only guards stale settings.
            Err(_) => builder.no_proxy(),
        },
    }
}

/// Build a client for a candidate setting so it can be validated before saving.
pub fn build_proxy_client(settings: &ProxySettings) -> Result<reqwest::Client, String> {
    settings.validate()?;
    builder_for(settings)
        .build()
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_custom_addresses_and_reports_game_endpoint() {
        let mut settings = ProxySettings {
            mode: ProxyMode::Custom,
            address: "http://user:secret@127.0.0.1:7890".into(),
        };
        settings.validate().unwrap();
        assert_eq!(settings.game_endpoint(), Some(("127.0.0.1".into(), 7890)));
        settings.address = "127.0.0.1:7890".into();
        assert!(settings.validate().is_err());
        settings.address = "socks5://127.0.0.1:7890".into();
        assert!(settings.validate().is_err());
        settings.mode = ProxyMode::Off;
        assert!(settings.validate().is_ok());
        assert_eq!(settings.game_endpoint(), None);
    }
}
