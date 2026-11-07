# WS2300 Weather Station Telemetry & Web UI

[![Github actions](https://github.com/sanpii/ws2300/workflows/.github/workflows/ci.yml/badge.svg)](https://github.com/sanpii/ws2300/actions?query=workflow%3A.github%2Fworkflows%2Fci.yml)
[![pipeline status](https://gitlab.com/sanpi/ws2300/badges/main/pipeline.svg)](https://gitlab.com/sanpi/ws2300/-/commits/main)

<p align="center">
  <img src="https://www.deltron.nl/cdn/shop/files/WS2300_a8c70825-c4e1-4817-a438-b1ebeaebade5.jpg?crop=center&height=1200&v=1731585808&width=1200" alt="La Crosse Technology WS2300 Weather Station" width="380">
</p>

A 100% pure Rust telemetry daemon, Web UI, and MQTT publisher for the **La Crosse Technology WS2300** weather station (originally inspired by [Open2300](http://www.lavrsen.dk/foswiki/bin/view/Open2300/WebHome) and part of the [Uncontrolled Mechanical Ventilation](https://hackaday.io/project/12708-uncontrolled-mechanical-ventilation) project).

---

## Key Features

- **Light Tropical-Blue Web UI**:
  - **Dashboard Default Page**: Displays real-time **Latest** and **Previous** sensor readings side-by-side along with exact reading timestamps, barometric tendency, station forecast, and a dynamic SVG wind compass.
  - **Distinctive Typography & Sharp Geometry**: Styled with `Chakra Petch`, `Space Grotesk`, and `JetBrains Mono` (`tabular-nums`) using crisp `3px` architectural corners and badges.
  - **Hamburger Navigation Drawer with Sub-Items**: Slide-out navigation menu providing quick jumps to the Dashboard and each Settings sub-section (`Serial Port & Metrics`, `MQTT Server & Topic`, `Stale Data Alerts`, `OIDC Authentication`, and `Out-of-Bounds Filters`).
  - **Custom Weather Logo & Icons**: Embedded vector weather iconography and favicon/manifest.
- **Service Control & Serial Port Management**:
  - **Start / Stop Service** directly from the top header bar.
  - **Serial Port Discovery**: Scan and select available serial ports (`/dev/ttyUSB0`, `/dev/ttyS0`, etc.) and configure polling intervals.
- **Include / Exclude `WIND` and `RAIN` Metrics**:
  - Independently toggle `include_wind` and `include_rain` in Settings (`config.yaml`).
  - When disabled, the respective metrics are skipped during 2400-baud serial polling (speeding up station reads), hidden from the Web UI Dashboard, and omitted from MQTT JSON payloads.
- **Out-of-Bounds Value Clamping**:
  - Configure valid `[min, max]` bounds for every sensor metric.
  - Automatically replaces out-of-bounds sensor spikes/error sentinels with the **previous valid value** during processing and flags clamped metrics on the Dashboard.
- **MQTT Telemetry Publisher**:
  - Publishes sanitized readings to an MQTT broker with configurable host, port, username/password authentication, topic, client ID, and retain flag.
- **Stale Data Watchdog & Multi-Channel Alerts**:
  - Triggers an alert when no valid values are received for `N` minutes (`stale_timeout_minutes`).
  - Supports **Email (SMTP)**, **Telegram Bot API**, and **Generic Webhook URLs** (compatible with Ntfy, Gotify, Slack, Discord, and Apprise).
- **Optional OIDC Authentication**:
  - Toggleable OpenID Connect (OIDC) login flow (`oidc.enabled`) using standard discovery or custom endpoints—identity-only with no complex permission/role setup required.
- **Atomic YAML Configuration**:
  - All settings are stored in `config.yaml` (see [`config.example.yaml`](config.example.yaml)).

---

## Building & Deploying the Latest Binary to `./bin` with Docker

All Rust source files are located in [`src/`](src/), while [`bin/`](bin/) is dedicated exclusively to compiled executables.

To compile the release binary inside Docker and export it directly to `./bin/ws2300` on your host:

```bash
./scripts/build-to-bin.sh
```

Or run the `docker build` export command directly:

```bash
docker build -f docker/Dockerfile --target export-bin --output type=local,dest=./bin .
```

---

## Starting & Viewing the UI Against a Mock WS2300 (Virtual PTY Serial)

Use the included [`scripts/run-mock-ui.sh`](scripts/run-mock-ui.sh) script to build `./bin/ws2300` (if not yet built), create a virtual serial port PTY (`/tmp/ws2300-mock-tty`) speaking the exact byte-for-byte WS2300 2400-baud nibble protocol, start the Web UI server, and open `http://localhost:8080` in your browser:

```bash
./scripts/run-mock-ui.sh
```

*(Pass `./scripts/run-mock-ui.sh --build` to force a fresh Docker rebuild of `./bin/ws2300` first.)*

---

## Deploying with Docker Compose

A ready-to-use Docker setup with inline build is located in the [`docker/`](docker/) directory:

```bash
cd docker
docker compose up -d --build
```

The Web UI will be available at `http://localhost:8080`, and configuration changes made in the UI will persist in `docker/config/config.yaml`.

To pass through a physical USB/RS232 serial adapter, uncomment the `devices` mapping in [`docker/docker-compose.yml`](docker/docker-compose.yml):

```yaml
    devices:
      - /dev/ttyUSB0:/dev/ttyUSB0
```

---

## Running Locally

### Web UI & Telemetry Service
```bash
./bin/ws2300 --config config.yaml
```

### One-Shot CLI JSON Output
For backwards-compatible single-read JSON output to stdout:
```bash
./bin/ws2300 --once /dev/ttyUSB0
```

---

## Development & Quality Checks

Run the full test suite, formatting verification, and strict Clippy lints:

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- --deny warnings
cargo test --all-targets --all-features
```
