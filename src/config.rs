use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
pub struct AppConfig {
    pub service: ServiceConfig,
    pub serial: SerialConfig,
    pub metrics: MetricsConfig,
    pub mqtt: MqttConfig,
    pub alerts: AlertsConfig,
    pub oidc: OidcConfig,
    pub bounds: BoundsConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct ServiceConfig {
    pub auto_start: bool,
    pub poll_interval_seconds: u64,
    pub bind_address: String,
    pub demo_mode: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct SerialConfig {
    pub port: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct MetricsConfig {
    pub include_wind: bool,
    pub include_rain: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct MqttConfig {
    pub enabled: bool,
    pub broker_host: String,
    pub broker_port: u16,
    pub username: String,
    pub password: String,
    pub topic: String,
    pub client_id: String,
    pub retain: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct AlertsConfig {
    pub enabled: bool,
    pub stale_timeout_minutes: u64,
    pub email: EmailAlertConfig,
    pub telegram: TelegramAlertConfig,
    pub webhook: WebhookAlertConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct EmailAlertConfig {
    pub enabled: bool,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_address: String,
    pub to_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
pub struct TelegramAlertConfig {
    pub enabled: bool,
    pub bot_token: String,
    pub chat_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
pub struct WebhookAlertConfig {
    pub enabled: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct OidcConfig {
    pub enabled: bool,
    pub issuer_url: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub session_secret: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FloatRange {
    pub min: f32,
    pub max: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UintRange {
    pub min: u32,
    pub max: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct BoundsConfig {
    pub replace_with_previous: bool,
    pub temperature_indoor: FloatRange,
    pub temperature_outdoor: FloatRange,
    pub dewpoint: FloatRange,
    pub humidity_indoor: UintRange,
    pub humidity_outdoor: UintRange,
    pub wind_speed: FloatRange,
    pub wind_dir: FloatRange,
    pub wind_chill: FloatRange,
    pub rain_1h: FloatRange,
    pub rain_24h: FloatRange,
    pub rain_total: FloatRange,
    pub pressure: FloatRange,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            auto_start: true,
            poll_interval_seconds: 60,
            bind_address: String::from("0.0.0.0:8080"),
            demo_mode: false,
        }
    }
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            port: String::from("/dev/ttyUSB0"),
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            include_wind: true,
            include_rain: true,
        }
    }
}

impl Default for MqttConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            broker_host: String::from("127.0.0.1"),
            broker_port: 1883,
            username: String::new(),
            password: String::new(),
            topic: String::from("weather/ws2300"),
            client_id: String::from("ws2300-station"),
            retain: true,
        }
    }
}

impl Default for AlertsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            stale_timeout_minutes: 15,
            email: EmailAlertConfig::default(),
            telegram: TelegramAlertConfig::default(),
            webhook: WebhookAlertConfig::default(),
        }
    }
}

impl Default for EmailAlertConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            smtp_host: String::from("smtp.example.com"),
            smtp_port: 587,
            smtp_username: String::new(),
            smtp_password: String::new(),
            from_address: String::from("ws2300@example.com"),
            to_address: String::from("admin@example.com"),
        }
    }
}

impl Default for OidcConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            issuer_url: String::from("https://auth.example.com"),
            authorization_endpoint: String::new(),
            token_endpoint: String::new(),
            userinfo_endpoint: String::new(),
            client_id: String::from("ws2300-web"),
            client_secret: String::new(),
            redirect_url: String::from("http://localhost:8080/auth/callback"),
            session_secret: String::from("change-me-session-secret-ws2300-key"),
        }
    }
}

impl Default for BoundsConfig {
    fn default() -> Self {
        Self {
            replace_with_previous: true,
            temperature_indoor: FloatRange {
                min: -10.0,
                max: 50.0,
            },
            temperature_outdoor: FloatRange {
                min: -40.0,
                max: 60.0,
            },
            dewpoint: FloatRange {
                min: -40.0,
                max: 60.0,
            },
            humidity_indoor: UintRange { min: 1, max: 100 },
            humidity_outdoor: UintRange { min: 1, max: 100 },
            wind_speed: FloatRange {
                min: 0.0,
                max: 75.0,
            },
            wind_dir: FloatRange {
                min: 0.0,
                max: 360.0,
            },
            wind_chill: FloatRange {
                min: -50.0,
                max: 60.0,
            },
            rain_1h: FloatRange {
                min: 0.0,
                max: 500.0,
            },
            rain_24h: FloatRange {
                min: 0.0,
                max: 1000.0,
            },
            rain_total: FloatRange {
                min: 0.0,
                max: 99999.0,
            },
            pressure: FloatRange {
                min: 850.0,
                max: 1100.0,
            },
        }
    }
}

impl AppConfig {
    pub fn load_or_create(path: &Path) -> Result<Self, String> {
        if path.exists() {
            let content = std::fs::read_to_string(path)
                .map_err(|e| format!("Failed to read config file {}: {e}", path.display()))?;
            let cfg: Self = serde_yaml::from_str(&content)
                .map_err(|e| format!("Failed to parse YAML config {}: {e}", path.display()))?;
            Ok(cfg)
        } else {
            let cfg = Self::default();
            cfg.save(path)?;
            Ok(cfg)
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
            && !parent.exists()
        {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {e}"))?;
        }
        let yaml = serde_yaml::to_string(self)
            .map_err(|e| format!("Failed to serialize config to YAML: {e}"))?;
        let tmp_path = path.with_extension("yaml.tmp");
        std::fs::write(&tmp_path, yaml)
            .map_err(|e| format!("Failed to write temporary config file: {e}"))?;
        std::fs::rename(&tmp_path, path)
            .map_err(|e| format!("Failed to atomically replace config file: {e}"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_yaml_roundtrip() {
        let cfg = AppConfig::default();
        let yaml = serde_yaml::to_string(&cfg).expect("serialize");
        let parsed: AppConfig = serde_yaml::from_str(&yaml).expect("deserialize");
        assert_eq!(cfg, parsed);
        assert!(parsed.metrics.include_wind);
        assert!(parsed.metrics.include_rain);
    }
}
