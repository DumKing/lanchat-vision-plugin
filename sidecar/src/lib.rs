pub mod face_monitor;
pub mod vision;

pub mod protocol {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FaceMonitorPolicyFrame {
        pub target_device_id: String,
        pub min_confidence: u8,
        pub body_min_confidence: u8,
        pub sample_fps: u8,
        pub consecutive_hits: u8,
        pub cooldown_seconds: u32,
        pub face_cooldown_seconds: u32,
        pub body_cooldown_seconds: u32,
        pub settings_locked: bool,
        pub version: i64,
        pub issued_by_device_id: String,
        pub issued_by_nickname: String,
        pub issued_at: i64,
    }
}

pub fn is_allowed_remote_update_url(value: &str) -> bool {
    reqwest::Url::parse(value).is_ok_and(|url| {
        url.scheme() == "https"
            && matches!(url.host_str(), Some("github.com" | "objects.githubusercontent.com"))
    })
}

pub fn authorized_update_request(client: &reqwest::Client, url: &str) -> reqwest::RequestBuilder {
    client.get(url).header("User-Agent", "LanChat-Vision-Plugin/0.8.0")
}
