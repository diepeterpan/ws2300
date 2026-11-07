use crate::alerts::dispatch_alert;
use crate::config::AppConfig;
use crate::mqtt::publish_reading;
use crate::processor::{ServiceStatusSnapshot, TimestampedReading, process_reading};
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Notify, RwLock, broadcast};
use tracing::{error, info, warn};
use ws2300::Data;

pub struct SharedState {
    pub config: RwLock<AppConfig>,
    pub config_path: PathBuf,
    pub running: RwLock<bool>,
    pub last_error: RwLock<Option<String>>,
    pub latest: RwLock<Option<TimestampedReading>>,
    pub previous: RwLock<Option<TimestampedReading>>,
    pub last_valid_data: RwLock<Option<Data>>,
    pub last_received_at: RwLock<Option<DateTime<Utc>>>,
    pub service_started_at: RwLock<DateTime<Utc>>,
    pub alert_active: RwLock<bool>,
    pub mqtt_connected: RwLock<bool>,
    pub wake_poller: Notify,
    pub sse_tx: broadcast::Sender<ServiceStatusSnapshot>,
}

impl SharedState {
    pub fn new(config: AppConfig, config_path: PathBuf) -> Arc<Self> {
        let auto_start = config.service.auto_start;
        let (sse_tx, _) = broadcast::channel(64);
        Arc::new(Self {
            config: RwLock::new(config),
            config_path,
            running: RwLock::new(auto_start),
            last_error: RwLock::new(None),
            latest: RwLock::new(None),
            previous: RwLock::new(None),
            last_valid_data: RwLock::new(None),
            last_received_at: RwLock::new(None),
            service_started_at: RwLock::new(Utc::now()),
            alert_active: RwLock::new(false),
            mqtt_connected: RwLock::new(false),
            wake_poller: Notify::new(),
            sse_tx,
        })
    }

    pub async fn snapshot(&self) -> ServiceStatusSnapshot {
        let cfg = self.config.read().await;
        ServiceStatusSnapshot {
            running: *self.running.read().await,
            demo_mode: cfg.service.demo_mode,
            last_error: self.last_error.read().await.clone(),
            latest: self.latest.read().await.clone(),
            previous: self.previous.read().await.clone(),
            alert_active: *self.alert_active.read().await,
            include_wind: cfg.metrics.include_wind,
            include_rain: cfg.metrics.include_rain,
            mqtt_connected: *self.mqtt_connected.read().await,
            serial_port: cfg.serial.port.clone(),
            poll_interval_seconds: cfg.service.poll_interval_seconds,
        }
    }

    pub async fn broadcast_snapshot(&self) {
        let snap = self.snapshot().await;
        let _ = self.sse_tx.send(snap);
    }
}

pub fn spawn_background_workers(state: Arc<SharedState>) {
    let poller_state = state.clone();
    tokio::spawn(async move {
        poller_loop(poller_state).await;
    });

    let watchdog_state = state;
    tokio::spawn(async move {
        watchdog_loop(watchdog_state).await;
    });
}

async fn poller_loop(state: Arc<SharedState>) {
    let mut cycle: u64 = 0;
    loop {
        let is_running = *state.running.read().await;
        if !is_running {
            state.wake_poller.notified().await;
            continue;
        }

        let cfg = state.config.read().await.clone();
        let include_wind = cfg.metrics.include_wind;
        let include_rain = cfg.metrics.include_rain;
        let port = cfg.serial.port.clone();
        let demo_mode = cfg.service.demo_mode;

        let read_result: Result<Data, String> = if demo_mode {
            Ok(generate_demo_reading(cycle, include_wind, include_rain))
        } else {
            tokio::task::spawn_blocking(move || {
                let dev = ws2300::Device::new(port).map_err(|e| e.to_string())?;
                dev.read_filtered(include_wind, include_rain)
                    .map_err(|e| e.to_string())
            })
            .await
            .map_err(|e| format!("Serial task join error: {e}"))
            .and_then(|r| r)
        };

        cycle = cycle.wrapping_add(1);

        match read_result {
            Ok(raw_data) => {
                let prev_valid = state.last_valid_data.read().await.clone();
                let (processed_data, clamped_fields) = process_reading(
                    raw_data,
                    prev_valid.as_ref(),
                    &cfg.bounds,
                    include_wind,
                    include_rain,
                );

                let now = Utc::now();
                let new_reading = TimestampedReading {
                    timestamp: now,
                    data: processed_data.clone(),
                    clamped_fields,
                };

                {
                    let current_latest = state.latest.read().await.clone();
                    if let Some(curr) = current_latest {
                        *state.previous.write().await = Some(curr);
                    }
                    *state.latest.write().await = Some(new_reading);
                    *state.last_valid_data.write().await = Some(processed_data.clone());
                    *state.last_received_at.write().await = Some(now);
                    *state.last_error.write().await = None;
                }

                // Clear stale alert if it was active
                let was_alert = {
                    let mut alert_guard = state.alert_active.write().await;
                    let prev_alert = *alert_guard;
                    *alert_guard = false;
                    prev_alert
                };

                if was_alert && cfg.alerts.enabled {
                    let _ = dispatch_alert(
                        &cfg.alerts,
                        "WS2300 Recovery: Telemetry Restored",
                        &format!("New sensor values received at {}", now.to_rfc3339()),
                    )
                    .await;
                }

                // Publish to MQTT if enabled
                if cfg.mqtt.enabled {
                    match publish_reading(&cfg.mqtt, &processed_data).await {
                        Ok(()) => {
                            *state.mqtt_connected.write().await = true;
                        }
                        Err(e) => {
                            *state.mqtt_connected.write().await = false;
                            warn!("MQTT publish error: {e}");
                        }
                    }
                } else {
                    *state.mqtt_connected.write().await = false;
                }

                state.broadcast_snapshot().await;
            }
            Err(err_msg) => {
                warn!("WS2300 read failed on {}: {}", cfg.serial.port, err_msg);
                *state.last_error.write().await = Some(err_msg);
                state.broadcast_snapshot().await;
            }
        }

        let sleep_secs = cfg.service.poll_interval_seconds.max(2);
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(sleep_secs)) => {}
            _ = state.wake_poller.notified() => {}
        }
    }
}

async fn watchdog_loop(state: Arc<SharedState>) {
    let mut interval = tokio::time::interval(Duration::from_secs(15));
    loop {
        interval.tick().await;

        let is_running = *state.running.read().await;
        if !is_running {
            continue;
        }

        let cfg = state.config.read().await.clone();
        if !cfg.alerts.enabled || cfg.alerts.stale_timeout_minutes == 0 {
            continue;
        }

        let reference_time = {
            let last_rx = *state.last_received_at.read().await;
            let started = *state.service_started_at.read().await;
            last_rx.unwrap_or(started)
        };

        let elapsed = Utc::now().signed_duration_since(reference_time);
        let threshold_secs = (cfg.alerts.stale_timeout_minutes as i64) * 60;

        if elapsed.num_seconds() >= threshold_secs {
            let already_alerted = *state.alert_active.read().await;
            if !already_alerted {
                *state.alert_active.write().await = true;
                let msg = format!(
                    "No valid WS2300 sensor values have been received for {} minutes (since {}). Serial port: {}",
                    cfg.alerts.stale_timeout_minutes,
                    reference_time.to_rfc3339(),
                    cfg.serial.port
                );
                info!("Triggering stale data alert: {msg}");
                if let Err(e) =
                    dispatch_alert(&cfg.alerts, "WS2300 Alert: Stale Sensor Data", &msg).await
                {
                    error!("Alert dispatch failed: {e}");
                }
                state.broadcast_snapshot().await;
            }
        }
    }
}

fn generate_demo_reading(cycle: u64, include_wind: bool, include_rain: bool) -> Data {
    let phase = (cycle % 20) as f32 * 0.1;
    // Periodically inject an out-of-bounds outdoor temperature spike on cycle % 5 == 3 to showcase clamping
    let outdoor_temp = if cycle > 0 && cycle % 5 == 3 {
        81.0 // Out-of-bounds sentinel that gets replaced with previous valid value
    } else {
        ((24.2 + phase) * 10.0).round() / 10.0
    };

    let (wind_speed, wind_dir, wind_direction, wind_chill) = if include_wind {
        (
            Some(((3.8 + phase * 0.5) * 10.0).round() / 10.0),
            Some(135.0),
            Some(String::from("SE")),
            Some(((23.6 + phase * 0.5) * 10.0).round() / 10.0),
        )
    } else {
        (None, None, None, None)
    };

    let (rain_1h, rain_24h, rain_total) = if include_rain {
        (Some(0.0), Some(1.5), Some(142.8))
    } else {
        (None, None, None)
    };

    Data {
        temperature_indoor: ((22.1 + phase * 0.2) * 10.0).round() / 10.0,
        temperature_outdoor: outdoor_temp,
        dewpoint: ((16.1 + phase * 0.3) * 10.0).round() / 10.0,
        humidity_indoor: 48,
        humidity_outdoor: 62 + (cycle % 3) as u32,
        wind_speed,
        wind_dir,
        wind_direction,
        wind_chill,
        rain_1h,
        rain_24h,
        rain_total,
        pressure: ((1014.0 + phase * 0.2) * 10.0).round() / 10.0,
        tendency: String::from("Rising"),
        forecast: String::from("Sunny"),
    }
}
