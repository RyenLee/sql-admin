use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Request, Response, header},
    middleware::Next,
};
use chrono::Utc;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Instant;
use tracing::{error, info, warn};
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";
const MAX_BODY_LOG_SIZE: usize = 4096;

const SENSITIVE_HEADERS: &[&str] = &[
    "authorization",
    "cookie",
    "set-cookie",
    "proxy-authorization",
    "www-authenticate",
];

const SENSITIVE_BODY_KEYS: &[&str] = &[
    "password",
    "token",
    "secret",
    "access_token",
    "refresh_token",
    "api_key",
    "private_key",
    "credential",
];

fn generate_request_id() -> String {
    Uuid::new_v4().to_string()
}

fn mask_headers(headers: &HeaderMap) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for (name, value) in headers.iter() {
        let key = name.as_str().to_lowercase();
        if SENSITIVE_HEADERS.contains(&key.as_str()) {
            result.insert(key, "******".to_string());
        } else if let Ok(v) = value.to_str() {
            result.insert(key, v.to_string());
        }
    }
    result
}

fn mask_sensitive_body(body: &str) -> String {
    if body.is_empty() {
        return body.to_string();
    }

    match serde_json::from_str::<Value>(body) {
        Ok(Value::Object(map)) => {
            let masked = mask_json_object(&map);
            serde_json::to_string(&masked).unwrap_or_else(|_| body.to_string())
        }
        _ => mask_non_json_body(body),
    }
}

fn mask_json_object(map: &serde_json::Map<String, Value>) -> Value {
    let mut masked = serde_json::Map::new();
    for (key, value) in map.iter() {
        let lower_key = key.to_lowercase();
        if SENSITIVE_BODY_KEYS.iter().any(|k| lower_key.contains(k)) {
            masked.insert(key.clone(), Value::String("******".to_string()));
        } else {
            masked.insert(key.clone(), value.clone());
        }
    }
    Value::Object(masked)
}

fn mask_non_json_body(body: &str) -> String {
    let mut result = body.to_string();
    for key in SENSITIVE_BODY_KEYS {
        let search = format!(r#""{}":"#, key);
        if let Some(pos) = result.find(search.as_str()) {
            let start = pos + search.len();
            let value_end = find_value_end(&result[start..]);
            let replacement = format!(r#""{}":"******""#, key);
            result = format!("{}{}{}", &result[..pos], replacement, &result[start + value_end..]);
        }
    }
    result
}

fn find_value_end(s: &str) -> usize {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return 0;
    }
    if bytes[0] == b'"' {
        let mut i = 1;
        while i < bytes.len() {
            if bytes[i] == b'\\' {
                i += 2;
                continue;
            }
            if bytes[i] == b'"' {
                return i + 1;
            }
            i += 1;
        }
        return bytes.len();
    }
    for (i, &b) in bytes.iter().enumerate() {
        if b == b',' || b == b'}' || b == b']' || b == b' ' || b == b'\n' {
            return i;
        }
    }
    bytes.len()
}

fn truncate_body(body: &str) -> String {
    if body.len() > MAX_BODY_LOG_SIZE {
        format!(
            "{}...[truncated, total {} bytes]",
            &body[..MAX_BODY_LOG_SIZE],
            body.len()
        )
    } else {
        body.to_string()
    }
}

pub async fn logging(req: Request<Body>, next: Next) -> Response<Body> {
    let request_id = req
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(generate_request_id);

    let method = req.method().clone();
    let uri = req.uri().clone();
    let version = req.version();
    let headers = mask_headers(req.headers());
    let query = uri.query().unwrap_or("").to_string();
    let path = uri.path().to_string();
    let timestamp = Utc::now().to_rfc3339();

    let span = tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %method,
        path = %path,
    );
    let _enter = span.enter();

    let (parts, body) = req.into_parts();
    let body_bytes = axum::body::to_bytes(body, 10 * 1024 * 1024)
        .await
        .unwrap_or_default();
    let body_str = String::from_utf8_lossy(&body_bytes).to_string();
    let masked_body = mask_sensitive_body(&body_str);
    let truncated_body = truncate_body(&masked_body);

    let content_type = parts
        .headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    info!(
        request_id = %request_id,
        module = "middleware::logging",
        event = "request_received",
        method = %method,
        path = %path,
        query = %query,
        version = ?version,
        headers = %serde_json::to_string(&headers).unwrap_or_default(),
        body = %truncated_body,
        content_type = %content_type,
        timestamp = %timestamp,
        "Incoming request"
    );

    let rebuilt_req = Request::from_parts(parts, Body::from(body_bytes));

    let start = Instant::now();
    let response = next.run(rebuilt_req).await;
    let elapsed_ms = start.elapsed().as_millis() as u64;

    let (parts, body) = response.into_parts();
    let status = parts.status;
    let response_headers = mask_headers(&parts.headers);

    let body_bytes = axum::body::to_bytes(body, 10 * 1024 * 1024)
        .await
        .unwrap_or_default();
    let body_str = String::from_utf8_lossy(&body_bytes).to_string();
    let truncated_response_body = truncate_body(&body_str);

    let response_timestamp = Utc::now().to_rfc3339();

    let log_level = if status.is_server_error() {
        tracing::Level::ERROR
    } else if status.is_client_error() {
        tracing::Level::WARN
    } else {
        tracing::Level::INFO
    };

    match log_level {
        tracing::Level::ERROR => {
            error!(
                request_id = %request_id,
                module = "middleware::logging",
                event = "response_sent",
                method = %method,
                path = %path,
                status_code = status.as_u16(),
                response_headers = %serde_json::to_string(&response_headers).unwrap_or_default(),
                response_body = %truncated_response_body,
                elapsed_ms = elapsed_ms,
                timestamp = %response_timestamp,
                "Request completed with server error"
            );
        }
        tracing::Level::WARN => {
            warn!(
                request_id = %request_id,
                module = "middleware::logging",
                event = "response_sent",
                method = %method,
                path = %path,
                status_code = status.as_u16(),
                response_headers = %serde_json::to_string(&response_headers).unwrap_or_default(),
                response_body = %truncated_response_body,
                elapsed_ms = elapsed_ms,
                timestamp = %response_timestamp,
                "Request completed with client error"
            );
        }
        _ => {
            info!(
                request_id = %request_id,
                module = "middleware::logging",
                event = "response_sent",
                method = %method,
                path = %path,
                status_code = status.as_u16(),
                response_headers = %serde_json::to_string(&response_headers).unwrap_or_default(),
                response_body = %truncated_response_body,
                elapsed_ms = elapsed_ms,
                timestamp = %response_timestamp,
                "Request completed"
            );
        }
    }

    let mut response = Response::from_parts(parts, Body::from(body_bytes));

    if let Ok(val) = HeaderValue::from_str(&request_id) {
        response.headers_mut().insert(REQUEST_ID_HEADER, val);
    }

    response
}