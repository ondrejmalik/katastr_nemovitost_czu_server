use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::json;
use tokio::try_join;

use crate::*;

#[derive(Debug, Deserialize)]
pub struct RizeniParams {
    pub id: Option<i32>,
    pub typ: Option<String>,
    pub cislo: Option<i32>,
    pub rok: Option<i32>,
}

pub async fn get_spravni_rizeni(
    State(pool): State<Pool>,
    Query(params): Query<RizeniParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let rizeni_id = if let Some(id) = params.id {
        id
    } else if let (Some(typ), Some(cislo), Some(rok)) = (params.typ, params.cislo, params.rok) {
        let client = pool.get().await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

        // 1. Get ID
        let row = client
            .query_opt(
                "SELECT * FROM fn_get_rizeni_id($1, $2, $3);",
                &[&typ, &cislo, &rok],
            )
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error: {}", e),
                )
            })?;

        match row {
            Some(row) => row.try_get(0).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Row parse error: {}", e),
                )
            })?,
            None => return Err((StatusCode::NOT_FOUND, "Rizeni not found".to_string())),
        }
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Missing parameters: either 'id' or 'typ', 'cislo', 'rok' must be provided".to_string(),
        ));
    };

    let pool_predmet = pool.clone();
    let task_predmet = async move {
        let start = std::time::Instant::now();
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&rizeni_id];
        let res = query_and_serialize_rizeni_predmet_poznamka(
            pool_predmet,
            "SELECT * FROM fn_get_rizeni_predmet_poznamka_by_id($1);",
            params,
        )
        .await;
        res.map(|v| (v, start.elapsed()))
    };

    let pool_ucastnici = pool.clone();
    let task_ucastnici = async move {
        let start = std::time::Instant::now();
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&rizeni_id];
        let res = query_and_serialize_rizeni_ucastnici(
            pool_ucastnici,
            "SELECT * FROM fn_get_ucastnici_rizeni_by_id($1);",
            params,
        )
        .await;
        res.map(|v| (v, start.elapsed()))
    };

    let pool_operace = pool.clone();
    let task_operace = async move {
        let start = std::time::Instant::now();
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&rizeni_id];
        let res = query_and_serialize_rizeni_operace(
            pool_operace,
            "SELECT * FROM fn_get_operace_rizeni_by_id($1);",
            params,
        )
        .await;
        res.map(|v| (v, start.elapsed()))
    };

    let ((predmet, t_predmet), (ucastnici, t_ucastnici), (operace, t_operace)) = try_join!(task_predmet, task_ucastnici, task_operace)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    if let (Some(p), Some(u), Some(o)) =
        (predmet.as_array(), ucastnici.as_array(), operace.as_array())
    {
        if p.is_empty() && u.is_empty() && o.is_empty() {
            return Err((
                StatusCode::NOT_FOUND,
                "Rizeni details not found".to_string(),
            ));
        }
    }

    let response_body = json!({
        "predmet": predmet,
        "ucastnici": ucastnici,
        "operace": operace,
    });

    let timing = format!(
        "predmet;dur={:.2}, ucastnici;dur={:.2}, operace;dur={:.2}",
        t_predmet.as_secs_f64() * 1000.0,
        t_ucastnici.as_secs_f64() * 1000.0,
        t_operace.as_secs_f64() * 1000.0
    );

    let mut response = Json(response_body).into_response();
    response.headers_mut().insert(
        axum::http::header::HeaderName::from_static("server-timing"),
        axum::http::HeaderValue::from_str(&timing).unwrap(),
    );

    Ok(response)
}
