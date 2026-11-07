use crate::alerts::dispatch_alert;
use crate::config::AppConfig;
use crate::oidc::{
    SESSION_COOKIE_NAME, STATE_COOKIE_NAME, build_authorize_redirect_url,
    create_session_cookie_value, exchange_code_for_subject, generate_pkce_pair, sign_value,
    verify_session_cookie_value, verify_signed_value,
};
use crate::service::SharedState;
use crate::ui_assets::{INDEX_HTML, LOGO_SVG, WEBMANIFEST_JSON};
use axum::body::Body;
use axum::extract::{Query, Request, State};
use axum::http::{StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::Utc;
use futures_util::StreamExt;
use serde::Deserialize;
use serde_json::json;
use std::convert::Infallible;
use std::sync::Arc;
use tokio_stream::wrappers::BroadcastStream;

pub fn build_router(state: Arc<SharedState>) -> Router {
    let protected = Router::new()
        .route("/", get(index_handler))
        .route("/api/status", get(status_handler))
        .route("/api/events", get(sse_handler))
        .route(
            "/api/config",
            get(get_config_handler).post(save_config_handler),
        )
        .route("/api/serial-ports", get(list_serial_ports_handler))
        .route("/api/service/start", post(start_service_handler))
        .route("/api/service/stop", post(stop_service_handler))
        .route("/api/alerts/test", post(test_alert_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            oidc_auth_middleware,
        ));

    Router::new()
        .merge(protected)
        .route("/favicon.svg", get(favicon_handler))
        .route("/site.webmanifest", get(manifest_handler))
        .route("/auth/login", get(oidc_login_handler))
        .route("/auth/callback", get(oidc_callback_handler))
        .route("/auth/logout", get(oidc_logout_handler))
        .with_state(state)
}

async fn oidc_auth_middleware(
    State(state): State<Arc<SharedState>>,
    jar: CookieJar,
    req: Request<Body>,
    next: Next,
) -> Response {
    let oidc_cfg = state.config.read().await.oidc.clone();
    if !oidc_cfg.enabled {
        return next.run(req).await;
    }

    if let Some(cookie) = jar.get(SESSION_COOKIE_NAME)
        && verify_session_cookie_value(cookie.value(), &oidc_cfg.session_secret).is_some()
    {
        return next.run(req).await;
    }

    let path = req.uri().path();
    if path.starts_with("/api/") {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "OIDC authentication required" })),
        )
            .into_response()
    } else {
        Redirect::to("/auth/login").into_response()
    }
}

async fn index_handler() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn favicon_handler() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "image/svg+xml")], LOGO_SVG)
}

async fn manifest_handler() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/manifest+json")],
        WEBMANIFEST_JSON,
    )
}

async fn status_handler(State(state): State<Arc<SharedState>>) -> impl IntoResponse {
    Json(state.snapshot().await)
}

async fn sse_handler(
    State(state): State<Arc<SharedState>>,
) -> Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>> {
    let rx = state.sse_tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|msg| async move {
        match msg {
            Ok(snap) => {
                let data = serde_json::to_string(&snap).unwrap_or_default();
                Some(Ok(Event::default().data(data)))
            }
            Err(_) => None,
        }
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn get_config_handler(State(state): State<Arc<SharedState>>) -> impl IntoResponse {
    let cfg = state.config.read().await.clone();
    Json(cfg)
}

async fn save_config_handler(
    State(state): State<Arc<SharedState>>,
    Json(new_cfg): Json<AppConfig>,
) -> impl IntoResponse {
    if let Err(e) = new_cfg.save(&state.config_path) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        )
            .into_response();
    }

    {
        let mut cfg_guard = state.config.write().await;
        *cfg_guard = new_cfg.clone();
    }

    // If wind or rain was disabled, also strip it from current cached readings immediately
    {
        let mut latest_guard = state.latest.write().await;
        if let Some(ref mut latest) = *latest_guard {
            if !new_cfg.metrics.include_wind {
                latest.data.wind_speed = None;
                latest.data.wind_dir = None;
                latest.data.wind_direction = None;
                latest.data.wind_chill = None;
            }
            if !new_cfg.metrics.include_rain {
                latest.data.rain_1h = None;
                latest.data.rain_24h = None;
                latest.data.rain_total = None;
            }
        }
    }

    state.wake_poller.notify_one();
    state.broadcast_snapshot().await;

    (StatusCode::OK, Json(json!({ "status": "ok" }))).into_response()
}

async fn list_serial_ports_handler(State(state): State<Arc<SharedState>>) -> impl IntoResponse {
    let configured_port = state.config.read().await.serial.port.clone();
    let mut ports: Vec<String> = Vec::new();

    if let Ok(available) = serialport::available_ports() {
        for p in available {
            ports.push(p.port_name);
        }
    }

    for candidate in ["/dev/ttyUSB0", "/dev/ttyUSB1", "/dev/ttyS0", "/dev/ttyACM0"] {
        if !ports.iter().any(|p| p == candidate) {
            ports.push(candidate.to_string());
        }
    }

    if !configured_port.is_empty() && !ports.iter().any(|p| p == &configured_port) {
        ports.insert(0, configured_port);
    }

    Json(ports)
}

async fn start_service_handler(State(state): State<Arc<SharedState>>) -> impl IntoResponse {
    {
        *state.running.write().await = true;
        *state.service_started_at.write().await = Utc::now();
        *state.last_error.write().await = None;
    }
    state.wake_poller.notify_one();
    let snap = state.snapshot().await;
    state.broadcast_snapshot().await;
    Json(snap)
}

async fn stop_service_handler(State(state): State<Arc<SharedState>>) -> impl IntoResponse {
    {
        *state.running.write().await = false;
    }
    let snap = state.snapshot().await;
    state.broadcast_snapshot().await;
    Json(snap)
}

async fn test_alert_handler(State(state): State<Arc<SharedState>>) -> impl IntoResponse {
    let alerts_cfg = state.config.read().await.alerts.clone();
    match dispatch_alert(
        &alerts_cfg,
        "WS2300 Test Alert",
        "This is a test notification from your WS2300 Weather Telemetry service.",
    )
    .await
    {
        Ok(results) => (StatusCode::OK, Json(json!({ "results": results }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err }))).into_response(),
    }
}

async fn oidc_login_handler(State(state): State<Arc<SharedState>>, jar: CookieJar) -> Response {
    let oidc_cfg = state.config.read().await.oidc.clone();
    if !oidc_cfg.enabled {
        return Redirect::to("/").into_response();
    }

    let random_nonce: u64 = rand::random();
    let state_token = format!("{random_nonce:016x}");
    let (code_verifier, code_challenge) = generate_pkce_pair();
    let state_cookie_payload = format!("{state_token}|{code_verifier}");
    let signed_state = sign_value(&state_cookie_payload, &oidc_cfg.session_secret);

    match build_authorize_redirect_url(&oidc_cfg, &state_token, &code_challenge).await {
        Ok(url) => {
            let mut cookie = Cookie::new(STATE_COOKIE_NAME, signed_state);
            cookie.set_path("/");
            cookie.set_http_only(true);
            cookie.set_same_site(SameSite::Lax);
            (jar.add(cookie), Redirect::to(&url)).into_response()
        }
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Html(format!("<h2>OIDC Configuration Error</h2><p>{e}</p>")),
        )
            .into_response(),
    }
}

#[derive(Deserialize, Debug)]
struct OidcCallbackQuery {
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    state: Option<String>,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    error_description: Option<String>,
}

async fn oidc_callback_handler(
    State(state): State<Arc<SharedState>>,
    jar: CookieJar,
    Query(query): Query<OidcCallbackQuery>,
) -> Response {
    let oidc_cfg = state.config.read().await.oidc.clone();

    if let Some(err) = query.error {
        let desc = query.error_description.unwrap_or_default();
        return (
            StatusCode::BAD_REQUEST,
            Html(format!(
                "<h2>OIDC Authentication Error</h2><p><strong>{}</strong>: {}</p><p><a href=\"/\">Return to Home</a></p>",
                html_escape(&err),
                html_escape(&desc)
            )),
        )
            .into_response();
    }

    let code = match query.code {
        Some(c) if !c.trim().is_empty() => c,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Html("<h2>OIDC Callback Error</h2><p>Missing authorization code in callback parameters.</p>".to_string()),
            )
                .into_response();
        }
    };

    let query_state = match query.state {
        Some(s) => s,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Html("<h2>OIDC Callback Error</h2><p>Missing state parameter in callback.</p>".to_string()),
            )
                .into_response();
        }
    };

    let state_cookie = match jar.get(STATE_COOKIE_NAME) {
        Some(c) => c.value().to_string(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Html("Missing OIDC state cookie".to_string()),
            )
                .into_response();
        }
    };

    let expected_payload = match verify_signed_value(&state_cookie, &oidc_cfg.session_secret) {
        Some(s) => s,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Html("Invalid OIDC state signature".to_string()),
            )
                .into_response();
        }
    };

    let (expected_state, code_verifier) = match expected_payload.split_once('|') {
        Some((st, cv)) => (st, cv),
        None => (expected_payload.as_str(), ""),
    };

    if expected_state != query_state {
        return (
            StatusCode::BAD_REQUEST,
            Html("OIDC state mismatch".to_string()),
        )
            .into_response();
    }

    match exchange_code_for_subject(&oidc_cfg, &code, code_verifier).await {
        Ok(subject) => {
            let session_val = create_session_cookie_value(&subject, &oidc_cfg.session_secret);
            let mut session_cookie = Cookie::new(SESSION_COOKIE_NAME, session_val);
            session_cookie.set_path("/");
            session_cookie.set_http_only(true);
            session_cookie.set_same_site(SameSite::Lax);

            let mut remove_state = Cookie::new(STATE_COOKIE_NAME, "");
            remove_state.set_path("/");

            (
                jar.remove(remove_state).add(session_cookie),
                Redirect::to("/"),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Html(format!("<h2>OIDC Login Failed</h2><p>{e}</p>")),
        )
            .into_response(),
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

async fn oidc_logout_handler(jar: CookieJar) -> Response {
    let mut remove_session = Cookie::new(SESSION_COOKIE_NAME, "");
    remove_session.set_path("/");
    (jar.remove(remove_session), Redirect::to("/auth/login")).into_response()
}
