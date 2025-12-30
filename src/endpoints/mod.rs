use axum::{
    Json,
    extract::{Query, State},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Instant;
use tokio::try_join;

use crate::{
    query_and_serialize_parcela, query_and_serialize_part_a, query_and_serialize_part_b,
    query_and_serialize_part_b_majitel, query_and_serialize_part_b_parcela,
    query_and_serialize_part_c, query_and_serialize_part_d, query_and_serialize_part_f,
};

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
) -> Result<Json<Value>, String> {
    let instant = Instant::now();
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

    let result = task.await.map_err(|e| format!("Database error: {}", e))?;
    println!("{:?}", instant.elapsed());
    println!("{:?}", result);
    Ok(Json(result))
}
#[derive(Debug, Deserialize)]
pub struct LvParams {
    pub katastralni_uzemi: String,
    pub cislo_lv: i32,
}

pub async fn get_lv_data(
    State(pool): State<Pool>,
    Query(params): Query<LvParams>,
) -> Result<Json<Value>, String> {
    let instant = Instant::now();
    let katastralni_uzemi = params.katastralni_uzemi;
    let cislo_lv = params.cislo_lv;

    let pool_a = pool.clone();
    let katastralni_uzemi_a = katastralni_uzemi.clone();
    let task_a = async move {
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_a, &cislo_lv];
        query_and_serialize_part_a(pool_a, "SELECT * FROM fn_get_lv_part_a($1, $2);", params).await
    };

    let pool_b = pool.clone();
    let katastralni_uzemi_b = katastralni_uzemi.clone();
    let task_b = async move {
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_b, &cislo_lv];
        query_and_serialize_part_b(pool_b, "SELECT * FROM fn_get_lv_part_b($1, $2);", params).await
    };

    let pool_b_parcela = pool.clone();
    let katastralni_uzemi_b_parcela = katastralni_uzemi.clone();
    let task_b_parcela = async move {
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_b_parcela, &cislo_lv];
        query_and_serialize_part_b_parcela(
            pool_b_parcela,
            "SELECT * FROM fn_get_lv_part_b_parcela($1, $2);",
            params,
        )
        .await
    };

    let pool_b_majitel = pool.clone();
    let katastralni_uzemi_b_majitel = katastralni_uzemi.clone();
    let task_b_majitel = async move {
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_b_majitel, &cislo_lv];
        query_and_serialize_part_b_majitel(
            pool_b_majitel,
            "SELECT * FROM fn_get_lv_part_b_majitel($1, $2);",
            params,
        )
        .await
    };

    let pool_c = pool.clone();
    let katastralni_uzemi_c = katastralni_uzemi.clone();
    let task_c = async move {
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_c, &cislo_lv];
        query_and_serialize_part_c(pool_c, "SELECT * FROM fn_get_lv_part_c($1, $2);", params).await
    };

    let pool_d = pool.clone();
    let katastralni_uzemi_d = katastralni_uzemi.clone();
    let task_d = async move {
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_d, &cislo_lv];
        query_and_serialize_part_d(pool_d, "SELECT * FROM fn_get_lv_part_d($1, $2);", params).await
    };

    let pool_f = pool.clone();
    let katastralni_uzemi_f = katastralni_uzemi.clone();
    let task_f = async move {
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] =
            &[&katastralni_uzemi_f, &cislo_lv];
        query_and_serialize_part_f(pool_f, "SELECT * FROM fn_get_lv_part_f($1, $2);", params).await
    };

    // Use try_join! to run them concurrently
    let (part_a, part_b, part_b_parcela, part_b_majitel, part_c, part_d, part_f) = try_join!(
        task_a,
        task_b,
        task_b_parcela,
        task_b_majitel,
        task_c,
        task_d,
        task_f
    )
    .map_err(|e| format!("Database error: {}", e))?;

    let response = json!({
        "part_a": part_a,
        "part_b": part_b,
        "part_b_parcela": part_b_parcela,
        "part_b_majitel": part_b_majitel,
        "part_c": part_c,
        "part_d": part_d,
        "part_f": part_f,
    });

    println!("{:?}", instant.elapsed());
    println!("{:?}", response);
    Ok(Json(response))
}
