use crate::config::OidcConfig;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

pub const SESSION_COOKIE_NAME: &str = "ws2300_session";
pub const STATE_COOKIE_NAME: &str = "ws2300_oidc_state";

pub fn generate_pkce_pair() -> (String, String) {
    let mut random_bytes = [0u8; 32];
    rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut random_bytes);
    let verifier = URL_SAFE_NO_PAD.encode(random_bytes);

    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    let challenge = URL_SAFE_NO_PAD.encode(hash);

    (verifier, challenge)
}

#[derive(Deserialize, Debug)]
struct OidcDiscoveryDocument {
    authorization_endpoint: String,
    token_endpoint: String,
    #[serde(default)]
    userinfo_endpoint: String,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    #[serde(default)]
    id_token: Option<String>,
}

pub fn sign_value(payload: &str, secret: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC accepts any key length");
    mac.update(payload.as_bytes());
    let sig = hex::encode(mac.finalize().into_bytes());
    format!("{payload}.{sig}")
}

pub fn verify_signed_value(signed: &str, secret: &str) -> Option<String> {
    let (payload, sig) = signed.rsplit_once('.')?;
    let expected = sign_value(payload, secret);
    if expected == signed && !sig.is_empty() {
        Some(payload.to_string())
    } else {
        None
    }
}

pub fn create_session_cookie_value(subject: &str, secret: &str) -> String {
    let exp = chrono::Utc::now().timestamp() + 86_400 * 7; // 7 days
    let raw = format!("{subject}|{exp}");
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, raw);
    sign_value(&encoded, secret)
}

pub fn verify_session_cookie_value(cookie_val: &str, secret: &str) -> Option<String> {
    let encoded = verify_signed_value(cookie_val, secret)?;
    let decoded_bytes =
        base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, encoded).ok()?;
    let decoded = String::from_utf8(decoded_bytes).ok()?;
    let (subject, exp_str) = decoded.rsplit_once('|')?;
    let exp: i64 = exp_str.parse().ok()?;
    if chrono::Utc::now().timestamp() <= exp {
        Some(subject.to_string())
    } else {
        None
    }
}

async fn resolve_endpoints(cfg: &OidcConfig) -> Result<(String, String, String), String> {
    if !cfg.authorization_endpoint.trim().is_empty() && !cfg.token_endpoint.trim().is_empty() {
        return Ok((
            cfg.authorization_endpoint.trim().to_string(),
            cfg.token_endpoint.trim().to_string(),
            cfg.userinfo_endpoint.trim().to_string(),
        ));
    }

    let issuer = cfg.issuer_url.trim().trim_end_matches('/');
    let discovery_url = format!("{issuer}/.well-known/openid-configuration");
    let client = reqwest::Client::new();
    let doc: OidcDiscoveryDocument = client
        .get(&discovery_url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch OIDC discovery document from {discovery_url}: {e}"))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse OIDC discovery JSON: {e}"))?;

    Ok((
        doc.authorization_endpoint,
        doc.token_endpoint,
        doc.userinfo_endpoint,
    ))
}

pub async fn build_authorize_redirect_url(
    cfg: &OidcConfig,
    state_token: &str,
    code_challenge: &str,
) -> Result<String, String> {
    let (auth_ep, _, _) = resolve_endpoints(cfg).await?;
    let url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
        auth_ep,
        urlencoding::encode(cfg.client_id.trim()),
        urlencoding::encode(cfg.redirect_url.trim()),
        urlencoding::encode("openid profile email"),
        urlencoding::encode(state_token),
        urlencoding::encode(code_challenge),
    );
    Ok(url)
}

pub async fn exchange_code_for_subject(
    cfg: &OidcConfig,
    code: &str,
    code_verifier: &str,
) -> Result<String, String> {
    let (_, token_ep, userinfo_ep) = resolve_endpoints(cfg).await?;
    let client = reqwest::Client::new();

    let mut params = vec![
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", cfg.redirect_url.trim()),
        ("client_id", cfg.client_id.trim()),
    ];

    if !code_verifier.trim().is_empty() {
        params.push(("code_verifier", code_verifier.trim()));
    }

    if !cfg.client_secret.trim().is_empty() {
        params.push(("client_secret", cfg.client_secret.trim()));
    }

    let resp = client
        .post(&token_ep)
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("OIDC token request failed: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("OIDC token endpoint returned {status}: {text}"));
    }

    let token_data: TokenResponse = resp
        .json()
        .await
        .map_err(|e| format!("Invalid OIDC token response JSON: {e}"))?;

    if let Some(id_token) = token_data.id_token {
        let parts: Vec<&str> = id_token.split('.').collect();
        if parts.len() >= 2
            && let Ok(bytes) =
                base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, parts[1])
            && let Ok(json_val) = serde_json::from_slice::<serde_json::Value>(&bytes)
        {
            if let Some(email) = json_val.get("email").and_then(|v| v.as_str()) {
                return Ok(email.to_string());
            }
            if let Some(sub) = json_val.get("sub").and_then(|v| v.as_str()) {
                return Ok(sub.to_string());
            }
        }
    }

    if !userinfo_ep.is_empty()
        && let Ok(u_resp) = client
            .get(&userinfo_ep)
            .bearer_auth(&token_data.access_token)
            .send()
            .await
        && let Ok(json_val) = u_resp.json::<serde_json::Value>().await
    {
        if let Some(email) = json_val.get("email").and_then(|v| v.as_str()) {
            return Ok(email.to_string());
        }
        if let Some(sub) = json_val.get("sub").and_then(|v| v.as_str()) {
            return Ok(sub.to_string());
        }
    }

    Ok(String::from("authenticated-oidc-user"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_cookie_signing_and_verification() {
        let secret = "super-secret-key";
        let cookie = create_session_cookie_value("user@example.com", secret);
        assert_eq!(
            verify_session_cookie_value(&cookie, secret),
            Some("user@example.com".to_string())
        );
        assert_eq!(verify_session_cookie_value(&cookie, "wrong-secret"), None);
    }
}
