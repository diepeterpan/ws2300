#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
BIN_PATH="${REPO_ROOT}/bin/ws2300"
MOCK_CONFIG="${REPO_ROOT}/config.mock.yaml"
MOCK_PTY="/tmp/ws2300-mock-tty"
UI_URL="http://localhost:8080"

if [[ ! -x "${BIN_PATH}" ]] || [[ "${1:-}" == "--build" ]]; then
  "${SCRIPT_DIR}/build-to-bin.sh"
fi

if [[ ! -f "${MOCK_CONFIG}" ]]; then
  cat > "${MOCK_CONFIG}" <<EOF
service:
  auto_start: true
  poll_interval_seconds: 5
  bind_address: "0.0.0.0:8080"
  demo_mode: false

serial:
  port: "${MOCK_PTY}"

metrics:
  include_wind: true
  include_rain: true

mqtt:
  enabled: false
  broker_host: "127.0.0.1"
  broker_port: 1883
  username: ""
  password: ""
  topic: "weather/ws2300"
  client_id: "ws2300-station"
  retain: true

alerts:
  enabled: false
  stale_timeout_minutes: 15

oidc:
  enabled: false
  issuer_url: "https://auth.example.com"
  client_id: "ws2300-web"
  client_secret: ""
  redirect_url: "http://localhost:8080/auth/callback"
  session_secret: "change-me-session-secret-ws2300-key"

bounds:
  replace_with_previous: true
  temperature_indoor: { min: -10.0, max: 50.0 }
  temperature_outdoor: { min: -40.0, max: 60.0 }
  dewpoint: { min: -40.0, max: 60.0 }
  humidity_indoor: { min: 1, max: 100 }
  humidity_outdoor: { min: 1, max: 100 }
  wind_speed: { min: 0.0, max: 75.0 }
  wind_dir: { min: 0.0, max: 360.0 }
  wind_chill: { min: -50.0, max: 60.0 }
  rain_1h: { min: 0.0, max: 500.0 }
  rain_24h: { min: 0.0, max: 1000.0 }
  rain_total: { min: 0.0, max: 99999.0 }
  pressure: { min: 850.0, max: 1100.0 }
EOF
fi

echo "================================================================"
echo " Starting WS2300 Web UI against Virtual Serial PTY Mock Station"
echo " Virtual Serial Port : ${MOCK_PTY}"
echo " Configuration File  : ${MOCK_CONFIG}"
echo " Web UI URL          : ${UI_URL}"
echo "================================================================"

(
  sleep 1.5
  if command -v xdg-open >/dev/null 2>&1; then
    xdg-open "${UI_URL}" >/dev/null 2>&1 || true
  elif command -v open >/dev/null 2>&1; then
    open "${UI_URL}" >/dev/null 2>&1 || true
  fi
) &

exec "${BIN_PATH}" --config "${MOCK_CONFIG}" --mock-serial "${MOCK_PTY}"

