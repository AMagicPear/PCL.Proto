//! Server-list ping results, mirroring `PCL.Core.Link.McPing.Model`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McPingVersionResult {
    pub name: String,
    #[serde(default)]
    pub protocol: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct McPingPlayerSampleResult {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct McPingPlayerResult {
    #[serde(default)]
    pub max: i32,
    #[serde(default)]
    pub online: i32,
    #[serde(default, rename = "sample")]
    pub samples: Vec<McPingPlayerSampleResult>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct McPingModInfoModResult {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct McPingModInfoResult {
    #[serde(default, rename = "type")]
    pub kind: String,
    #[serde(default, rename = "modList")]
    pub mod_list: Vec<McPingModInfoModResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McPingResult {
    pub version: McPingVersionResult,
    #[serde(default)]
    pub players: McPingPlayerResult,
    /// Chat component: servers may return a string or a structured object.
    #[serde(default)]
    pub description: Value,
    #[serde(default)]
    pub favicon: Option<String>,
    #[serde(default)]
    pub latency: u64,
    #[serde(default, rename = "modinfo")]
    pub mod_info: Option<McPingModInfoResult>,
    #[serde(default)]
    pub prevents_chat_reports: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_vanilla_status_payload() {
        let result: McPingResult = serde_json::from_value(json!({
            "version": {"name": "1.21.1", "protocol": 767},
            "players": {
                "max": 20,
                "online": 3,
                "sample": [{"name": "Player", "id": "uuid"}]
            },
            "description": {"text": "Hello", "color": "gold"},
            "favicon": "data:image/png;base64,AAAA",
            "modinfo": {"type": "FML", "modList": [{"id": "forge", "version": "47.2"}]},
            "preventsChatReports": true
        }))
        .unwrap();
        assert_eq!(result.version.name, "1.21.1");
        assert_eq!(result.players.online, 3);
        assert_eq!(result.players.samples[0].name, "Player");
        assert_eq!(result.description["text"], "Hello");
        assert_eq!(result.mod_info.unwrap().mod_list[0].id, "forge");
        assert_eq!(result.prevents_chat_reports, Some(true));
    }

    #[test]
    fn tolerates_missing_optional_sections() {
        let result: McPingResult = serde_json::from_value(json!({
            "version": {"name": "unknown"},
            "description": "plain"
        }))
        .unwrap();
        assert_eq!(result.version.protocol, 0);
        assert_eq!(result.players.online, 0);
        assert!(result.players.samples.is_empty());
        assert!(result.mod_info.is_none());
    }
}
