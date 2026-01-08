use axum::{
    Extension,
    extract::{Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use rand::{distr::Alphanumeric, Rng};
use std::time::Instant;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct AuthenticateParams {
    pub password: String,
}

pub async fn get_authenticate(
    State(_pool): State<Pool>,
    Query(params): Query<AuthenticateParams>,
    state: Extension<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let password = params.password.clone();
    let hashed = state.password.clone();

    let verify_res =
        tokio::task::spawn_blocking(move || bcrypt::verify(password.as_str(), hashed.as_str()))
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Task join error: {}", e),
                )
            })?
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Bcrypt error: {}", e),
                )
            })?;

    if verify_res {
        // Generate random session ID
        let session_id: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        // Store session in AppState
        {
            let mut sessions = state.sessions.write().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;
            sessions.insert(session_id.clone(), Instant::now());
        }

        let mut headers = HeaderMap::new();
        // Set cookie with HttpOnly flag; path=/ so it's sent for all endpoints
        // Max-Age set to 3600 seconds (1 hour)
        let cookie_val = format!(
            "katastr_session={}; Max-Age=3600; Path=/; HttpOnly",
            session_id
        );
        headers.insert(
            axum::http::header::SET_COOKIE,
            HeaderValue::from_str(&cookie_val).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Header error: {}", e),
                )
            })?,
        );
        Ok((StatusCode::OK, headers))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid hash".to_string()))
    }
}
