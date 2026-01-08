use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;

use crate::*;

#[derive(Debug, Deserialize)]
pub struct ParcelaParams {
    pub katastralni_uzemi: String,
    pub parcelni_cislo: i32,
    pub cast_parcely: i32,
    pub je_stavebni: bool,
}

pub async fn get_parceala_data(
    State(pool): State<Pool>,
    Query(params): Query<ParcelaParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let katastralni_uzemi = params.katastralni_uzemi;
    let parcelni_cislo = params.parcelni_cislo;
    let cast_parcely = params.cast_parcely;
    let je_stavebni = params.je_stavebni;
    let task = async move {
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[
            &katastralni_uzemi,
            &je_stavebni,
            &parcelni_cislo,
            &cast_parcely,
        ];
        query_and_serialize_parcela(
            pool,
            "SELECT * FROM fn_get_parcela($1, $2, $3, $4);",
            params,
        )
        .await
    };

    let result = task.await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;
    if let Some(arr) = result.as_array() {
        if arr.is_empty() {
            return Err((StatusCode::NOT_FOUND, "Parcela not found".to_string()));
        }
    }
    Ok(Json(result))
}
