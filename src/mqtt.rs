use crate::config::MqttConfig;
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use std::time::Duration;
use tracing::{error, info, warn};
use ws2300::Data;

pub async fn publish_reading(cfg: &MqttConfig, data: &Data) -> Result<(), String> {
    if !cfg.enabled {
        return Ok(());
    }

    let payload = serde_json::to_string(data)
        .map_err(|e| format!("Failed to serialize telemetry for MQTT: {e}"))?;

    let client_id = if cfg.client_id.trim().is_empty() {
        "ws2300-station"
    } else {
        cfg.client_id.trim()
    };

    let mut mqttoptions = MqttOptions::new(client_id, cfg.broker_host.trim(), cfg.broker_port);
    mqttoptions.set_keep_alive(Duration::from_secs(15));

    if !cfg.username.trim().is_empty() {
        mqttoptions.set_credentials(cfg.username.trim(), &cfg.password);
    }

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
        .publish(
            cfg.topic.trim(),
            QoS::AtLeastOnce,
            cfg.retain,
            payload.as_bytes(),
        )
        .await
        .map_err(|e| format!("Failed to enqueue MQTT publish: {e}"))?;

    let deadline = tokio::time::Instant::now() + Duration::from_secs(6);
    loop {
        match tokio::time::timeout_at(deadline, eventloop.poll()).await {
            Ok(Ok(Event::Incoming(Incoming::PubAck(_)))) => {
                info!("Published WS2300 telemetry to MQTT topic '{}'", cfg.topic);
                let _ = client.disconnect().await;
                return Ok(());
            }
            Ok(Ok(_)) => continue,
            Ok(Err(e)) => {
                error!("MQTT connection error: {e}");
                return Err(format!("MQTT error: {e}"));
            }
            Err(_) => {
                warn!(
                    "Timed out waiting for MQTT PubAck from {}:{}",
                    cfg.broker_host, cfg.broker_port
                );
                return Err(String::from("MQTT publish timed out"));
            }
        }
    }
}
