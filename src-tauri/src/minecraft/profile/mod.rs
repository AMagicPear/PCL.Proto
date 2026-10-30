//! Launcher profiles, mirroring `PCL.Core.Minecraft.Profile.Models`.
//!
//! The C# core keeps a mutable [`McProfile`] whose stable identity is the
//! profile id. Rust keeps the same fields, but represents the profile kind with
//! an enum-derived `profile_type` and uses `Option` for values that are only
//! meaningful to one authentication provider.

use serde::{Deserialize, Serialize};

pub mod skin;

/// Authentication provider of a launcher profile.
///
/// Mirrors `PCL.Core.Minecraft.Profile.Models.ProfileType`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileType {
    #[serde(rename = "microsoft")]
    Microsoft,
    #[default]
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "authlib")]
    Authlib,
    #[serde(rename = "yggdrasilConnect")]
    YggdrasilConnect,
}

/// Full persisted launcher profile.
///
/// Mirrors `PCL.Core.Minecraft.Profile.Models.McProfile` / `SafeProfile`.
/// Provider-only fields stay optional so offline profiles do not accumulate
/// meaningless credentials.
#[derive(Clone, Debug, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct McProfile {
    pub profile_id: String,
    pub user_name: String,
    pub access_token: String,
    pub uuid: String,
    pub token_type: String,
    pub profile_type: ProfileType,

    pub refresh_token: String,
    pub client_token: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub description: String,
    pub skin_head_id: String,
    pub raw_json: String,

    // Authlib / Yggdrasil
    pub server: Option<String>,
    pub server_name: Option<String>,
    pub login_name: Option<String>,
    pub password: Option<String>,

    // OAuth / OpenID provider metadata
    pub provider: Option<String>,
    pub discovery_address: Option<String>,
    pub id_token: Option<String>,

    // Authentication details kept by the launcher for Xbox/Minecraft arguments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_homepage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_meta: Option<String>,
}

/// Legacy `Account` enum persisted by Proto before the `McProfile` migration.
#[derive(Deserialize)]
enum LegacyAccount {
    Offline {
        username: String,
        uuid: String,
    },
    Yggdrasil {
        username: String,
        uuid: String,
        access_token: String,
    },
    Microsoft {
        username: String,
        uuid: String,
        access_token: String,
        refresh_token: String,
        xuid: String,
    },
    AuthlibInjector {
        api_location: String,
        server_name: String,
        server_homepage: String,
        server_meta: String,
        access_token: String,
        uuid: String,
        username: String,
    },
}

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct ModernProfile {
    profile_id: String,
    user_name: String,
    access_token: String,
    uuid: String,
    token_type: String,
    profile_type: ProfileType,
    refresh_token: String,
    client_token: String,
    expires_at: Option<chrono::DateTime<chrono::Utc>>,
    description: String,
    skin_head_id: String,
    raw_json: String,
    server: Option<String>,
    server_name: Option<String>,
    login_name: Option<String>,
    password: Option<String>,
    provider: Option<String>,
    discovery_address: Option<String>,
    id_token: Option<String>,
    xuid: Option<String>,
    server_homepage: Option<String>,
    server_meta: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ProfileCompat {
    Legacy(LegacyAccount),
    Modern(ModernProfile),
}

impl From<ModernProfile> for McProfile {
    fn from(value: ModernProfile) -> Self {
        Self {
            profile_id: value.profile_id,
            user_name: value.user_name,
            access_token: value.access_token,
            uuid: value.uuid,
            token_type: if value.token_type.is_empty() {
                "Bearer".into()
            } else {
                value.token_type
            },
            profile_type: value.profile_type,
            refresh_token: value.refresh_token,
            client_token: value.client_token,
            expires_at: value.expires_at,
            description: value.description,
            skin_head_id: value.skin_head_id,
            raw_json: value.raw_json,
            server: value.server,
            server_name: value.server_name,
            login_name: value.login_name,
            password: value.password,
            provider: value.provider,
            discovery_address: value.discovery_address,
            id_token: value.id_token,
            xuid: value.xuid,
            server_homepage: value.server_homepage,
            server_meta: value.server_meta,
        }
    }
}

impl From<LegacyAccount> for McProfile {
    fn from(value: LegacyAccount) -> Self {
        match value {
            LegacyAccount::Offline { username, uuid } => McProfile::offline(username, uuid),
            LegacyAccount::Yggdrasil {
                username,
                uuid,
                access_token,
            } => McProfile::yggdrasil(username, uuid, access_token),
            LegacyAccount::Microsoft {
                username,
                uuid,
                access_token,
                refresh_token,
                xuid,
            } => McProfile::microsoft(username, uuid, access_token, refresh_token, xuid),
            LegacyAccount::AuthlibInjector {
                api_location,
                server_name,
                server_homepage,
                server_meta,
                access_token,
                uuid,
                username,
            } => McProfile::authlib(
                api_location,
                server_name,
                server_homepage,
                server_meta,
                access_token,
                uuid,
                username,
            ),
        }
    }
}

impl<'de> Deserialize<'de> for McProfile {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match ProfileCompat::deserialize(deserializer)? {
            ProfileCompat::Legacy(legacy) => Ok(legacy.into()),
            ProfileCompat::Modern(modern) => Ok(modern.into()),
        }
    }
}

impl Default for McProfile {
    fn default() -> Self {
        Self {
            profile_id: String::new(),
            user_name: String::new(),
            access_token: String::new(),
            uuid: String::new(),
            token_type: "Bearer".into(),
            profile_type: ProfileType::Offline,
            refresh_token: String::new(),
            client_token: String::new(),
            expires_at: None,
            description: String::new(),
            skin_head_id: String::new(),
            raw_json: String::new(),
            server: None,
            server_name: None,
            login_name: None,
            password: None,
            provider: None,
            discovery_address: None,
            id_token: None,
            xuid: None,
            server_homepage: None,
            server_meta: None,
        }
    }
}

impl McProfile {
    pub fn offline(user_name: impl Into<String>, uuid: impl Into<String>) -> Self {
        let user_name = user_name.into();
        let uuid = uuid.into();
        Self {
            profile_id: uuid.clone(),
            user_name,
            uuid,
            ..Self::default()
        }
    }

    pub fn yggdrasil(
        user_name: impl Into<String>,
        uuid: impl Into<String>,
        access_token: impl Into<String>,
    ) -> Self {
        let user_name = user_name.into();
        let uuid = uuid.into();
        Self {
            profile_id: uuid.clone(),
            user_name,
            uuid,
            access_token: access_token.into(),
            profile_type: ProfileType::YggdrasilConnect,
            ..Self::default()
        }
    }

    pub fn microsoft(
        user_name: impl Into<String>,
        uuid: impl Into<String>,
        access_token: impl Into<String>,
        refresh_token: impl Into<String>,
        xuid: impl Into<String>,
    ) -> Self {
        let user_name = user_name.into();
        let uuid = uuid.into();
        Self {
            profile_id: uuid.clone(),
            user_name,
            uuid,
            access_token: access_token.into(),
            refresh_token: refresh_token.into(),
            profile_type: ProfileType::Microsoft,
            xuid: Some(xuid.into()),
            ..Self::default()
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn authlib(
        api_location: impl Into<String>,
        server_name: impl Into<String>,
        server_homepage: impl Into<String>,
        server_meta: impl Into<String>,
        access_token: impl Into<String>,
        uuid: impl Into<String>,
        user_name: impl Into<String>,
    ) -> Self {
        let uuid = uuid.into();
        Self {
            profile_id: uuid.clone(),
            user_name: user_name.into(),
            uuid,
            access_token: access_token.into(),
            profile_type: ProfileType::Authlib,
            server: Some(api_location.into()),
            server_name: Some(server_name.into()),
            server_homepage: Some(server_homepage.into()),
            server_meta: Some(server_meta.into()),
            ..Self::default()
        }
    }

    /// Display name used by launch arguments and the UI.
    pub fn username(&self) -> &str {
        &self.user_name
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn access_token(&self) -> Option<&str> {
        (!self.access_token.is_empty()).then_some(self.access_token.as_str())
    }

    pub fn xuid(&self) -> Option<&str> {
        self.xuid.as_deref().filter(|value| !value.is_empty())
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at
            .is_some_and(|expires| expires <= chrono::Utc::now() + chrono::Duration::minutes(1))
    }

    /// Validate the parts of a profile that are required to launch a game.
    pub fn validate_for_launch(&self) -> Result<(), String> {
        if self.user_name.trim().is_empty() {
            return Err("Profile has no player name".into());
        }
        if self.uuid.trim().is_empty() {
            return Err("Profile has no UUID".into());
        }
        if matches!(
            self.profile_type,
            ProfileType::Microsoft | ProfileType::YggdrasilConnect | ProfileType::Authlib
        ) && self.access_token.is_empty()
        {
            return Err("Profile has no access token".into());
        }
        Ok(())
    }
}

/// UI-safe projection of a profile, mirroring `PCL.Core.Minecraft.Profile.Models.SafeProfile`.
///
/// Tokens and provider secrets stay in the backend; the webview only receives
/// identity and display fields.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SafeProfile {
    pub profile_id: String,
    pub user_name: String,
    pub uuid: String,
    pub token_type: String,
    pub profile_type: ProfileType,
    pub description: String,
    pub skin_head_id: String,
    pub server: Option<String>,
    pub server_name: Option<String>,
    pub server_homepage: Option<String>,
}

impl From<&McProfile> for SafeProfile {
    fn from(profile: &McProfile) -> Self {
        Self {
            profile_id: profile.profile_id.clone(),
            user_name: profile.user_name.clone(),
            uuid: profile.uuid.clone(),
            token_type: profile.token_type.clone(),
            profile_type: profile.profile_type,
            description: profile.description.clone(),
            skin_head_id: profile.skin_head_id.clone(),
            server: profile.server.clone(),
            server_name: profile.server_name.clone(),
            server_homepage: profile.server_homepage.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_profile_never_exposes_tokens() {
        let profile = McProfile::microsoft("Player", "uuid", "access", "refresh", "xuid");
        let safe = SafeProfile::from(&profile);
        let value = serde_json::to_value(&safe).unwrap();
        assert_eq!(value["userName"], "Player");
        assert_eq!(value["profileType"], "microsoft");
        assert!(value.get("accessToken").is_none());
        assert!(value.get("refreshToken").is_none());
        assert!(value.get("xuid").is_none());
    }

    #[test]
    fn legacy_account_enums_migrate_to_profiles() {
        let offline: McProfile = serde_json::from_value(serde_json::json!({
            "Offline": {"username": "Player", "uuid": "uuid"}
        }))
        .unwrap();
        assert_eq!(offline.user_name, "Player");
        assert_eq!(offline.profile_type, ProfileType::Offline);

        let microsoft: McProfile = serde_json::from_value(serde_json::json!({
            "Microsoft": {
                "username": "Player",
                "uuid": "uuid",
                "access_token": "access",
                "refresh_token": "refresh",
                "xuid": "xuid"
            }
        }))
        .unwrap();
        assert_eq!(microsoft.profile_type, ProfileType::Microsoft);
        assert_eq!(microsoft.access_token(), Some("access"));
        assert_eq!(microsoft.refresh_token, "refresh");
        assert_eq!(microsoft.xuid(), Some("xuid"));

        let authlib: McProfile = serde_json::from_value(serde_json::json!({
            "AuthlibInjector": {
                "api_location": "https://example.org/api",
                "server_name": "Example",
                "server_homepage": "https://example.org",
                "server_meta": "meta",
                "access_token": "access",
                "uuid": "uuid",
                "username": "Player"
            }
        }))
        .unwrap();
        assert_eq!(authlib.profile_type, ProfileType::Authlib);
        assert_eq!(authlib.server.as_deref(), Some("https://example.org/api"));
        assert_eq!(authlib.server_meta.as_deref(), Some("meta"));
    }

    #[test]
    fn modern_profile_round_trips() {
        let profile = McProfile::yggdrasil("Player", "uuid", "token");
        let value = serde_json::to_value(&profile).unwrap();
        let restored: McProfile = serde_json::from_value(value).unwrap();
        assert_eq!(restored.profile_type, ProfileType::YggdrasilConnect);
        assert_eq!(restored.access_token(), Some("token"));
    }

    #[test]
    fn profile_kinds_keep_stable_names() {
        for (kind, name) in [
            (ProfileType::Offline, "offline"),
            (ProfileType::Microsoft, "microsoft"),
            (ProfileType::Authlib, "authlib"),
            (ProfileType::YggdrasilConnect, "yggdrasilConnect"),
        ] {
            assert_eq!(serde_json::to_value(kind).unwrap(), serde_json::json!(name));
        }
    }
}
