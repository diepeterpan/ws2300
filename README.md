# WS2300 Weather Station Telemetry & Web UI

[![Github actions](https://github.com/sanpii/ws2300/workflows/.github/workflows/ci.yml/badge.svg)](https://github.com/sanpii/ws2300/actions?query=workflow%3A.github%2Fworkflows%2Fci.yml)
[![pipeline status](https://gitlab.com/sanpi/ws2300/badges/main/pipeline.svg)](https://gitlab.com/sanpi/ws2300/-/commits/main)

<p align="center">
  <img src="https://www.deltron.nl/cdn/shop/files/WS2300_a8c70825-c4e1-4817-a438-b1ebeaebade5.jpg?crop=center&height=1200&v=1731585808&width=1200" alt="La Crosse Technology WS2300 Weather Station" width="380">
</p>

A 100% pure Rust telemetry daemon, Web UI, and MQTT publisher for the **La Crosse Technology WS2300** weather station (originally inspired by [Open2300](http://www.lavrsen.dk/foswiki/bin/view/Open2300/WebHome) and part of the [Uncontrolled Mechanical Ventilation](https://hackaday.io/project/12708-uncontrolled-mechanical-ventilation) project).

<p align="center">
  <img src="doc/dashboard.png" alt="WS2300 Web UI Dashboard" width="850">
</p>

*WS2300 Web UI dashboard displaying real-time sensor metrics, previous comparisons, hardware serial link status, and wind compass.*

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
- **Optional OIDC Authentication & Single Sign-On**:
  - Toggleable OpenID Connect (OIDC) login flow (`oidc.enabled`) supporting standard discovery, authorization code flow with **RFC 7636 PKCE (S256)**, and authenticated user indicator in the header.
- **Atomic YAML Configuration**:
  - All settings are stored in `config.yaml` (see [`config.example.yaml`](config.example.yaml)).

---

## OpenID Connect (OIDC) & Single Sign-On

WS2300 includes built-in OIDC identity authentication to protect the Web UI and API routes without complex RBAC configuration.

### Security & PKCE Support
- **Proof Key for Code Exchange (RFC 7636 PKCE)**: WS2300 implements the `S256` code challenge and verifier flow during the authorization code exchange, protecting against code interception attacks.
- **Header Profile Pill**: Once logged in, the active username or email is displayed in the top header beside a user icon, along with a quick-logout shortcut.

### System CA Trust & Private PKI (Cargo.toml Configuration)
WS2300 uses `reqwest` compiled with `rustls-tls-native-roots`:
```toml
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls-native-roots"] }
```
Unlike standard Rust TLS configurations that only bundle public Mozilla root certificates, this allows WS2300 to **inherit the host operating system's trust store** (such as `/etc/ssl/certs/ca-certificates.crt`).

To trust an internal or private Certificate Authority (such as for internal Authelia or Keycloak instances):
```bash
# Debian / Ubuntu host or Docker container:
sudo cp my-private-ca.crt /usr/local/share/ca-certificates/
sudo update-ca-certificates
```

> [!NOTE]
> Ensure internal CA and server certificates comply with RFC 5280 by including `X509v3 Authority Key Identifier` (AKID) and `X509v3 Subject Key Identifier` extensions for strict TLS handshake validation.

### Sample Authelia Client Configuration

Below is an anonymized sample client definition to add to Authelia's `configuration.yml` under `identity_providers.oidc.clients`:

```yaml
identity_providers:
  oidc:
    clients:
      - client_id: 'ws2300-web'
        client_name: 'WS2300 Weather Station'
        consent_mode: 'implicit'
        # Generate with: authelia crypto hash generate pbkdf2 --variant sha512
        client_secret: '$pbkdf2-sha512$310000$exampleSalt$exampleHash...'
        public: false
        authorization_policy: 'one_factor' # Or 'two_factor' depending on your security policy
        scopes:
          - 'openid'
          - 'profile'
          - 'email'
          - 'groups'
        redirect_uris:
          - 'https://ws2300.example.com/auth/callback'
        grant_types:
          - 'authorization_code'
        response_types:
          - 'code'
        access_token_signed_response_alg: 'none'
        userinfo_signed_response_alg: 'none'
        token_endpoint_auth_method: 'client_secret_post'
        require_pkce: true
        pkce_challenge_method: 'S256'
```

### Corresponding WS2300 `config.yaml` Settings

```yaml
oidc:
  enabled: true
  issuer_url: "https://authelia.example.com"
  client_id: "ws2300-web"
  client_secret: "yourPlainTextClientSecret"
  redirect_url: "https://ws2300.example.com/auth/callback"
  session_secret: "generate-a-secure-random-32-byte-hex-string"
```

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
