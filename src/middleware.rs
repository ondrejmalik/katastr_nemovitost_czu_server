use crate::models::AppState;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};
use std::time::Instant;
use tracing::info;

pub async fn track_latency(req: Request, next: Next) -> impl IntoResponse {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let response = next.run(req).await;
    let duration = start.elapsed();

    // Log unconditionally; filtering is handled by the global tracing subscriber
    info!("Request: {} {} took {:?}", method, uri, duration);

    let mut response = response.into_response();
    let duration_ms = duration.as_secs_f64() * 1000.0;
    let header_value = format!("total;dur={:.2}", duration_ms);

    response.headers_mut().append(
        axum::http::header::HeaderName::from_static("server-timing"),
        axum::http::HeaderValue::from_str(&header_value).unwrap(),
    );

    response
}

pub async fn require_auth_cookie(state: AppState, req: Request, next: Next) -> impl IntoResponse {
    // Allow safe methods without auth

    let method = req.method().clone();

    let path = req.uri().path().to_string();

    info!("Auth middleware entry: {} {}", method, path);

    if method == axum::http::Method::GET || method == axum::http::Method::OPTIONS {
        info!("Auth middleware: allowing safe method {}", method);

        return next.run(req).await;
    }

    // Allow health and auth endpoints without the cookie

    if path == "/health" || path == "/auth" {
        info!("Auth middleware: allowing public path {}", path);

        return next.run(req).await;
    }

    // Parse cookie header for `katastr_session`

    let cookie_header = req
        .headers()
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok());

    info!("Auth middleware: cookie header = {:?}", cookie_header);

    let cookie_value = cookie_header.and_then(|s| {
        s.split(';').map(|pair| pair.trim()).find_map(|pair| {
            let mut parts = pair.splitn(2, '=');

            let key = parts.next()?;

            let val = parts.next()?;

            if key == "katastr_session" {
                Some(val.to_string())
            } else {
                None
            }
        })
    });

    let session_id = match cookie_value {
        Some(v) => {
            info!("Auth middleware: found cookie katastr_session={v}");

            v
        }

        None => {
            info!("Auth middleware: missing auth cookie - returning 401");

            return (StatusCode::UNAUTHORIZED, "Missing auth cookie".to_string()).into_response();
        }
    };

    // Check if session exists in memory

    let is_valid = {
        let sessions = match state.sessions.read() {
            Ok(s) => s,

            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string())
                    .into_response();
            }
        };

        sessions.contains_key(&session_id)
    };

    if is_valid {
        info!("Auth middleware: valid session");

        next.run(req).await
    } else {
        info!("Auth middleware: invalid session - returning 401");

        (StatusCode::UNAUTHORIZED, "Invalid session".to_string()).into_response()
    }
}
