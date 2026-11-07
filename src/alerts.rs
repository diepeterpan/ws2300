use crate::config::{AlertsConfig, EmailAlertConfig, TelegramAlertConfig, WebhookAlertConfig};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use serde_json::json;
use tracing::{error, info};

pub async fn dispatch_alert(
    cfg: &AlertsConfig,
    subject: &str,
    body: &str,
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();
    let mut errors = Vec::new();

    if cfg.email.enabled {
        match send_email(&cfg.email, subject, body).await {
            Ok(()) => {
                info!("Sent alert email to {}", cfg.email.to_address);
                results.push(format!("Email sent to {}", cfg.email.to_address));
            }
            Err(e) => {
                error!("Failed to send email alert: {e}");
                errors.push(format!("Email error: {e}"));
            }
        }
    }

    if cfg.telegram.enabled {
        match send_telegram(&cfg.telegram, subject, body).await {
            Ok(()) => {
                info!("Sent Telegram alert to chat {}", cfg.telegram.chat_id);
                results.push(format!("Telegram sent to {}", cfg.telegram.chat_id));
            }
            Err(e) => {
                error!("Failed to send Telegram alert: {e}");
                errors.push(format!("Telegram error: {e}"));
            }
        }
    }

    if cfg.webhook.enabled {
        match send_webhook(&cfg.webhook, subject, body).await {
            Ok(()) => {
                info!("Sent Webhook alert to {}", cfg.webhook.url);
                results.push(format!("Webhook sent to {}", cfg.webhook.url));
            }
            Err(e) => {
                error!("Failed to send Webhook alert: {e}");
                errors.push(format!("Webhook error: {e}"));
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors.join("; "));
    }

    if results.is_empty() {
        return Err(String::from(
            "No alert channels are enabled in configuration",
        ));
    }

    Ok(results)
}

async fn send_email(cfg: &EmailAlertConfig, subject: &str, body: &str) -> Result<(), String> {
    let email = Message::builder()
        .from(
            cfg.from_address
                .parse()
                .map_err(|e| format!("Invalid from_address: {e}"))?,
        )
        .to(cfg
            .to_address
            .parse()
            .map_err(|e| format!("Invalid to_address: {e}"))?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())
        .map_err(|e| format!("Failed to construct email: {e}"))?;

    let sec = cfg.smtp_security.to_ascii_lowercase();
    let mut builder = match sec.as_str() {
        "plain" | "none" => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&cfg.smtp_host)
            .port(cfg.smtp_port),
        "tls" | "ssl" => AsyncSmtpTransport::<Tokio1Executor>::relay(&cfg.smtp_host)
            .map_err(|e| format!("SMTP TLS error: {e}"))?
            .port(cfg.smtp_port),
        _ => {
            // Default to STARTTLS (or TLS if port is explicitly 465 and unspecified)
            if sec.is_empty() && cfg.smtp_port == 465 {
                AsyncSmtpTransport::<Tokio1Executor>::relay(&cfg.smtp_host)
                    .map_err(|e| format!("SMTP relay error: {e}"))?
            } else {
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&cfg.smtp_host)
                    .map_err(|e| format!("SMTP STARTTLS error: {e}"))?
                    .port(cfg.smtp_port)
            }
        }
    };

    if !cfg.smtp_username.trim().is_empty() {
        let creds = Credentials::new(cfg.smtp_username.clone(), cfg.smtp_password.clone());
        builder = builder.credentials(creds);
    }

    let mailer = builder.build();
    mailer
        .send(email)
        .await
        .map_err(|e| format!("SMTP send failed: {e}"))?;

    Ok(())
}

async fn send_telegram(cfg: &TelegramAlertConfig, subject: &str, body: &str) -> Result<(), String> {
    if cfg.bot_token.trim().is_empty() || cfg.chat_id.trim().is_empty() {
        return Err(String::from("Telegram bot_token and chat_id are required"));
    }

    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        cfg.bot_token.trim()
    );
    let text = format!("*{subject}*\n{body}");

    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .json(&json!({
            "chat_id": cfg.chat_id.trim(),
            "text": text,
            "parse_mode": "Markdown"
        }))
        .send()
        .await
        .map_err(|e| format!("Telegram HTTP request failed: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err_text = resp.text().await.unwrap_or_default();
        return Err(format!("Telegram API returned {status}: {err_text}"));
    }

    Ok(())
}

async fn send_webhook(cfg: &WebhookAlertConfig, subject: &str, body: &str) -> Result<(), String> {
    if cfg.url.trim().is_empty() {
        return Err(String::from("Webhook URL is empty"));
    }

    let client = reqwest::Client::new();
    let resp = client
        .post(cfg.url.trim())
        .json(&json!({
            "title": subject,
            "message": body,
            "text": format!("{subject}: {body}"),
            "content": format!("**{subject}**\n{body}")
        }))
        .send()
        .await
        .map_err(|e| format!("Webhook HTTP request failed: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err_text = resp.text().await.unwrap_or_default();
        return Err(format!("Webhook returned {status}: {err_text}"));
    }

    Ok(())
}
