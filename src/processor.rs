use crate::config::{BoundsConfig, FloatRange, UintRange};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ws2300::Data;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimestampedReading {
    pub timestamp: DateTime<Utc>,
    pub data: Data,
    pub clamped_fields: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServiceStatusSnapshot {
    pub running: bool,
    pub demo_mode: bool,
    pub last_error: Option<String>,
    pub latest: Option<TimestampedReading>,
    pub previous: Option<TimestampedReading>,
    pub alert_active: bool,
    pub include_wind: bool,
    pub include_rain: bool,
    pub mqtt_connected: bool,
    pub serial_port: String,
    pub poll_interval_seconds: u64,
}

fn check_f32(
    field_name: &str,
    val: f32,
    range: &FloatRange,
    prev_val: Option<f32>,
    replace: bool,
    clamped: &mut Vec<String>,
) -> f32 {
    if val < range.min || val > range.max || val.is_nan() {
        clamped.push(field_name.to_string());
        if replace && let Some(p) = prev_val {
            return p;
        }
    }
    val
}

fn check_u32(
    field_name: &str,
    val: u32,
    range: &UintRange,
    prev_val: Option<u32>,
    replace: bool,
    clamped: &mut Vec<String>,
) -> u32 {
    if val < range.min || val > range.max {
        clamped.push(field_name.to_string());
        if replace && let Some(p) = prev_val {
            return p;
        }
    }
    val
}

/// Validates `incoming` data against `bounds`. If a field is out of bounds and
/// `bounds.replace_with_previous` is true, replaces it with the corresponding value
/// from `previous_valid` (if available) and records the field name in `clamped_fields`.
pub fn process_reading(
    mut incoming: Data,
    previous_valid: Option<&Data>,
    bounds: &BoundsConfig,
    include_wind: bool,
    include_rain: bool,
) -> (Data, Vec<String>) {
    let mut clamped = Vec::new();
    let replace = bounds.replace_with_previous;

    incoming.temperature_indoor = check_f32(
        "temperature_indoor",
        incoming.temperature_indoor,
        &bounds.temperature_indoor,
        previous_valid.map(|p| p.temperature_indoor),
        replace,
        &mut clamped,
    );

    incoming.temperature_outdoor = check_f32(
        "temperature_outdoor",
        incoming.temperature_outdoor,
        &bounds.temperature_outdoor,
        previous_valid.map(|p| p.temperature_outdoor),
        replace,
        &mut clamped,
    );

    incoming.dewpoint = check_f32(
        "dewpoint",
        incoming.dewpoint,
        &bounds.dewpoint,
        previous_valid.map(|p| p.dewpoint),
        replace,
        &mut clamped,
    );

    incoming.humidity_indoor = check_u32(
        "humidity_indoor",
        incoming.humidity_indoor,
        &bounds.humidity_indoor,
        previous_valid.map(|p| p.humidity_indoor),
        replace,
        &mut clamped,
    );

    incoming.humidity_outdoor = check_u32(
        "humidity_outdoor",
        incoming.humidity_outdoor,
        &bounds.humidity_outdoor,
        previous_valid.map(|p| p.humidity_outdoor),
        replace,
        &mut clamped,
    );

    incoming.pressure = check_f32(
        "pressure",
        incoming.pressure,
        &bounds.pressure,
        previous_valid.map(|p| p.pressure),
        replace,
        &mut clamped,
    );

    if include_wind {
        if let Some(ws) = incoming.wind_speed {
            incoming.wind_speed = Some(check_f32(
                "wind_speed",
                ws,
                &bounds.wind_speed,
                previous_valid.and_then(|p| p.wind_speed),
                replace,
                &mut clamped,
            ));
        }
        if let Some(wd) = incoming.wind_dir {
            let new_wd = check_f32(
                "wind_dir",
                wd,
                &bounds.wind_dir,
                previous_valid.and_then(|p| p.wind_dir),
                replace,
                &mut clamped,
            );
            if clamped.iter().any(|c| c == "wind_dir")
                && replace
                && let Some(prev_dir_str) = previous_valid.and_then(|p| p.wind_direction.clone())
            {
                incoming.wind_direction = Some(prev_dir_str);
            }
            incoming.wind_dir = Some(new_wd);
        }
        if let Some(wc) = incoming.wind_chill {
            incoming.wind_chill = Some(check_f32(
                "wind_chill",
                wc,
                &bounds.wind_chill,
                previous_valid.and_then(|p| p.wind_chill),
                replace,
                &mut clamped,
            ));
        }
    } else {
        incoming.wind_speed = None;
        incoming.wind_dir = None;
        incoming.wind_direction = None;
        incoming.wind_chill = None;
    }

    if include_rain {
        if let Some(r1) = incoming.rain_1h {
            incoming.rain_1h = Some(check_f32(
                "rain_1h",
                r1,
                &bounds.rain_1h,
                previous_valid.and_then(|p| p.rain_1h),
                replace,
                &mut clamped,
            ));
        }
        if let Some(r24) = incoming.rain_24h {
            incoming.rain_24h = Some(check_f32(
                "rain_24h",
                r24,
                &bounds.rain_24h,
                previous_valid.and_then(|p| p.rain_24h),
                replace,
                &mut clamped,
            ));
        }
        if let Some(rt) = incoming.rain_total {
            incoming.rain_total = Some(check_f32(
                "rain_total",
                rt,
                &bounds.rain_total,
                previous_valid.and_then(|p| p.rain_total),
                replace,
                &mut clamped,
            ));
        }
    } else {
        incoming.rain_1h = None;
        incoming.rain_24h = None;
        incoming.rain_total = None;
    }

    (incoming, clamped)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Data {
        Data {
            temperature_indoor: 22.0,
            temperature_outdoor: 18.5,
            dewpoint: 11.2,
            humidity_indoor: 45,
            humidity_outdoor: 68,
            wind_speed: Some(3.4),
            wind_dir: Some(180.0),
            wind_direction: Some("S".to_string()),
            wind_chill: Some(17.8),
            rain_1h: Some(0.0),
            rain_24h: Some(2.5),
            rain_total: Some(120.4),
            pressure: 1013.2,
            tendency: "Steady".to_string(),
            forecast: "Sunny".to_string(),
        }
    }

    #[test]
    fn test_out_of_bounds_replacement() {
        let prev = sample_data();
        let mut spike = sample_data();
        spike.temperature_outdoor = 81.0; // WS2300 sensor error sentinel
        spike.humidity_outdoor = 110;
        spike.rain_1h = Some(999.9);

        let bounds = BoundsConfig::default();
        let (cleaned, clamped) = process_reading(spike, Some(&prev), &bounds, true, true);

        assert_eq!(cleaned.temperature_outdoor, 18.5);
        assert_eq!(cleaned.humidity_outdoor, 68);
        assert_eq!(cleaned.rain_1h, Some(0.0));
        assert!(clamped.contains(&"temperature_outdoor".to_string()));
        assert!(clamped.contains(&"humidity_outdoor".to_string()));
        assert!(clamped.contains(&"rain_1h".to_string()));
    }

    #[test]
    fn test_exclude_wind_and_rain_from_data_and_json() {
        let raw = sample_data();
        let bounds = BoundsConfig::default();
        let (cleaned, _) = process_reading(raw, None, &bounds, false, false);

        assert!(cleaned.wind_speed.is_none());
        assert!(cleaned.rain_1h.is_none());

        let json = serde_json::to_string(&cleaned).expect("json");
        assert!(!json.contains("wind_speed"));
        assert!(!json.contains("rain_1h"));
        assert!(json.contains("temperature_outdoor"));
    }
}
