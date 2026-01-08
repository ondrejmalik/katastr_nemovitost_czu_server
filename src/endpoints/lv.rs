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
pub struct LvParams {
    pub katastralni_uzemi: String,
    pub cislo_lv: i32,
}

pub async fn get_lv_data(
    State(pool): State<Pool>,
    Query(params): Query<LvParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let katastralni_uzemi = params.katastralni_uzemi;
    let cislo_lv = params.cislo_lv;

    let pool_a = pool.clone();
    let katastralni_uzemi_a = katastralni_uzemi.clone();
    let task_a = async move {
        let start = std::time::Instant::now();
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_a, &cislo_lv];
        let res = query_and_serialize_part_a(pool_a, "SELECT * FROM fn_get_lv_part_a($1, $2);", params).await;
        res.map(|v| (v, start.elapsed()))
    };

    let pool_b = pool.clone();
    let katastralni_uzemi_b = katastralni_uzemi.clone();
    let task_b = async move {
        let start = std::time::Instant::now();
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_b, &cislo_lv];
        let res = query_and_serialize_part_b(pool_b, "SELECT * FROM fn_get_lv_part_b($1, $2);", params).await;
        res.map(|v| (v, start.elapsed()))
    };

    let pool_b_parcela = pool.clone();
    let katastralni_uzemi_b_parcela = katastralni_uzemi.clone();
    let task_b_parcela = async move {
        let start = std::time::Instant::now();
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_b_parcela, &cislo_lv];
        let res = query_and_serialize_part_b_parcela(
            pool_b_parcela,
            "SELECT * FROM fn_get_lv_part_b_parcela($1, $2);",
            params,
        )
        .await;
        res.map(|v| (v, start.elapsed()))
    };

    let pool_b_majitel = pool.clone();
    let katastralni_uzemi_b_majitel = katastralni_uzemi.clone();
    let task_b_majitel = async move {
        let start = std::time::Instant::now();
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_b_majitel, &cislo_lv];
        let res = query_and_serialize_part_b_majitel(
            pool_b_majitel,
            "SELECT * FROM fn_get_lv_part_b_majitel($1, $2);",
            params,
        )
        .await;
        res.map(|v| (v, start.elapsed()))
    };

    let pool_c = pool.clone();
    let katastralni_uzemi_c = katastralni_uzemi.clone();
    let task_c = async move {
        let start = std::time::Instant::now();
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_c, &cislo_lv];
        let res = query_and_serialize_part_c(pool_c, "SELECT * FROM fn_get_lv_part_c($1, $2);", params).await;
        res.map(|v| (v, start.elapsed()))
    };

    let pool_d = pool.clone();
    let katastralni_uzemi_d = katastralni_uzemi.clone();
    let task_d = async move {
        let start = std::time::Instant::now();
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_d, &cislo_lv];
        let res = query_and_serialize_part_d(pool_d, "SELECT * FROM fn_get_lv_part_d($1, $2);", params).await;
        res.map(|v| (v, start.elapsed()))
    };

    let pool_f = pool.clone();
    let katastralni_uzemi_f = katastralni_uzemi.clone();
    let task_f = async move {
        let start = std::time::Instant::now();
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_f, &cislo_lv];
        let res = query_and_serialize_part_f(pool_f, "SELECT * FROM fn_get_lv_part_f($1, $2);", params).await;
        res.map(|v| (v, start.elapsed()))
    };

    // Use try_join! to run them concurrently
    let (
        (part_a, t_a),
        (part_b, t_b),
        (part_b_parcela, t_bp),
        (part_b_majitel, t_bm),
        (part_c, t_c),
        (part_d, t_d),
        (part_f, t_f)
    ) = try_join!(
        task_a,
        task_b,
        task_b_parcela,
        task_b_majitel,
        task_c,
        task_d,
        task_f
    )
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    if let Some(arr) = part_a.as_array() {
        if arr.is_empty() {
            return Err((StatusCode::NOT_FOUND, "LV not found".to_string()));
        }
    }

    let response_body = json!({
        "part_a": part_a,
        "part_b": part_b,
        "part_b_parcela": part_b_parcela,
        "part_b_majitel": part_b_majitel,
        "part_c": part_c,
        "part_d": part_d,
        "part_f": part_f,
    });

    let timing = format!(
        "part_a;dur={:.2}, part_b;dur={:.2}, part_b_parcela;dur={:.2}, part_b_majitel;dur={:.2}, part_c;dur={:.2}, part_d;dur={:.2}, part_f;dur={:.2}",
        t_a.as_secs_f64() * 1000.0,
        t_b.as_secs_f64() * 1000.0,
        t_bp.as_secs_f64() * 1000.0,
        t_bm.as_secs_f64() * 1000.0,
        t_c.as_secs_f64() * 1000.0,
        t_d.as_secs_f64() * 1000.0,
        t_f.as_secs_f64() * 1000.0
    );

    let mut response = Json(response_body).into_response();
    response.headers_mut().insert(
        axum::http::header::HeaderName::from_static("server-timing"),
        axum::http::HeaderValue::from_str(&timing).unwrap(),
    );

    Ok(response)
}
