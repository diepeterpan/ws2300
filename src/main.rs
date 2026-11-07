#![warn(warnings)]

mod alerts;
mod config;
mod mock_station;
mod mqtt;
mod oidc;
mod processor;
mod service;
mod ui_assets;
mod web;

use clap::Parser;
use config::AppConfig;
use mock_station::spawn_virtual_ws2300_pty;
use service::{SharedState, spawn_background_workers};
use std::path::PathBuf;
use tracing::info;
use web::build_router;

#[derive(Parser, Debug)]
#[command(
    name = "ws2300",
    about = "WS2300 Weather Station Web UI, MQTT & Telemetry Service"
)]
struct Opt {
    /// Path to YAML configuration file
    #[arg(short, long, env = "WS2300_CONFIG", default_value = "config.yaml")]
    config: PathBuf,

    /// Start a virtual PTY serial port emulating a real WS2300 weather station at the given path
    #[arg(long, env = "WS2300_MOCK_SERIAL")]
    mock_serial: Option<PathBuf>,

    /// Optional one-shot serial device path (prints JSON once and exits for CLI backwards compatibility)
    #[arg(long)]
    once: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let opt = Opt::parse();

    if let Some(device_path) = opt.once {
        let ws2300 = ws2300::Device::new(device_path)?;
        let data = ws2300.read_all()?;
        let json = serde_json::to_string(&data)?;
        println!("{json}");
        return Ok(());
    }

    let mut app_config = AppConfig::load_or_create(&opt.config)?;

    if let Some(mock_pty_path) = opt.mock_serial {
        spawn_virtual_ws2300_pty(mock_pty_path.clone())?;
        app_config.serial.port = mock_pty_path.to_string_lossy().to_string();
        app_config.service.demo_mode = false;
    }

    let bind_addr = app_config.service.bind_address.clone();

    info!(
        "Loaded configuration from {} (Serial port: {}, Wind included: {}, Rain included: {})",
        opt.config.display(),
        app_config.serial.port,
        app_config.metrics.include_wind,
        app_config.metrics.include_rain
    );

    let state = SharedState::new(app_config, opt.config);
    spawn_background_workers(state.clone());

    let app = build_router(state);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("WS2300 Web UI listening on http://{bind_addr}");

    axum::serve(listener, app).await?;
    Ok(())
}
