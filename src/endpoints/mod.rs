use axum::response::IntoResponse;
use axum::{
    Extension, Json,
    extract::{Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::try_join;

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
#[derive(Debug, Deserialize)]
pub struct LvParams {
    pub katastralni_uzemi: String,
    pub cislo_lv: i32,
}

pub async fn get_lv_data(
    State(pool): State<Pool>,
    Query(params): Query<LvParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
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

    let response = json!({
        "part_a": part_a,
        "part_b": part_b,
        "part_b_parcela": part_b_parcela,
        "part_b_majitel": part_b_majitel,
        "part_c": part_c,
        "part_d": part_d,
        "part_f": part_f,
    });

    Ok(Json(response))
}

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
) -> Result<Json<Value>, (StatusCode, String)> {
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
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&rizeni_id];
        query_and_serialize_rizeni_predmet_poznamka(
            pool_predmet,
            "SELECT * FROM fn_get_rizeni_predmet_poznamka_by_id($1);",
            params,
        )
        .await
    };

    let pool_ucastnici = pool.clone();
    let task_ucastnici = async move {
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&rizeni_id];
        query_and_serialize_rizeni_ucastnici(
            pool_ucastnici,
            "SELECT * FROM fn_get_ucastnici_rizeni_by_id($1);",
            params,
        )
        .await
    };

    let pool_operace = pool.clone();
    let task_operace = async move {
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&rizeni_id];
        query_and_serialize_rizeni_operace(
            pool_operace,
            "SELECT * FROM fn_get_operace_rizeni_by_id($1);",
            params,
        )
        .await
    };

    let (predmet, ucastnici, operace) = try_join!(task_predmet, task_ucastnici, task_operace)
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

    let response = json!({
        "predmet": predmet,
        "ucastnici": ucastnici,
        "operace": operace,
    });

    Ok(Json(response))
}
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
        let mut headers = HeaderMap::new();
        // Set cookie with HttpOnly flag; path=/ so it's sent for all endpoints
        // Max-Age set to 3600 seconds (1 hour)
        let cookie_val = format!(
            "katastr_pw={}; Max-Age=3600; Path=/; HttpOnly",
            params.password
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
pub async fn get_health(State(_pool): State<Pool>) -> Result<(), String> {
    Ok(())
}

// --- Generic CRUD Handlers ---

macro_rules! crud_handlers {
    ($name:ident, $struct:ident, $new_struct:ident, $get_fn:ident, $create_fn:ident, $update_fn:ident, $delete_fn:ident) => {
        pub async fn $name(State(pool): State<Pool>) -> Result<Json<Vec<$struct>>, String> {
            let result = $get_fn(pool).await.map_err(|e| format!("Database error: {}", e))?;
            Ok(Json(result))
        }

        pub mod $name {
            use super::*;
            pub async fn create(
                State(pool): State<Pool>,
                Json(item): Json<$new_struct>,
            ) -> Result<Json<Value>, String> {
                let result = $create_fn(pool, item).await.map_err(|e| format!("Database error: {}", e))?;
                Ok(Json(json!({ "rows_affected": result })))
            }

            pub async fn update(
                State(pool): State<Pool>,
                Json(item): Json<$struct>,
            ) -> Result<Json<Value>, String> {
                let result = $update_fn(pool, item).await.map_err(|e| format!("Database error: {}", e))?;
                Ok(Json(json!({ "rows_affected": result })))
            }

            #[derive(Debug, Deserialize)]
            pub struct DeleteParams {
                pub id: i32,
            }

            pub async fn delete(
                State(pool): State<Pool>,
                Query(params): Query<DeleteParams>,
            ) -> Result<Json<Value>, (StatusCode, String)> {
                let result = $delete_fn(pool, params.id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
                if result == 0 {
                    return Err((StatusCode::NOT_FOUND, "Item not found".to_string()));
                }
                Ok(Json(json!({ "rows_affected": result })))
            }
        }
    };
}

// Handlers for tables with composite keys need special handling or a different macro
macro_rules! crud_handlers_composite_2 {
    ($name:ident, $struct:ident, $new_struct:ident, $get_fn:ident, $create_fn:ident, $update_fn:ident, $delete_fn:ident, $key1:ident, $key2:ident) => {
        pub async fn $name(State(pool): State<Pool>) -> Result<Json<Vec<$struct>>, String> {
            let result = $get_fn(pool).await.map_err(|e| format!("Database error: {}", e))?;
            Ok(Json(result))
        }

        pub mod $name {
            use super::*;
            pub async fn create(
                State(pool): State<Pool>,
                Json(item): Json<$new_struct>,
            ) -> Result<Json<Value>, String> {
                let result = $create_fn(pool, item).await.map_err(|e| format!("Database error: {}", e))?;
                Ok(Json(json!({ "rows_affected": result })))
            }

            pub async fn update(
                State(pool): State<Pool>,
                Json(item): Json<$struct>,
            ) -> Result<Json<Value>, String> {
                let result = $update_fn(pool, item).await.map_err(|e| format!("Database error: {}", e))?;
                Ok(Json(json!({ "rows_affected": result })))
            }

            #[derive(Debug, Deserialize)]
            pub struct DeleteParams {
                pub $key1: i32,
                pub $key2: i32,
            }

            pub async fn delete(
                State(pool): State<Pool>,
                Query(params): Query<DeleteParams>,
            ) -> Result<Json<Value>, (StatusCode, String)> {
                let result = $delete_fn(pool, params.$key1, params.$key2).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
                if result == 0 {
                    return Err((StatusCode::NOT_FOUND, "Item not found".to_string()));
                }
                Ok(Json(json!({ "rows_affected": result })))
            }
        }
    };
}

macro_rules! crud_handlers_composite_3 {
    ($name:ident, $struct:ident, $new_struct:ident, $get_fn:ident, $create_fn:ident, $delete_fn:ident, $key1:ident, $key2:ident, $key3:ident) => {
        pub async fn $name(State(pool): State<Pool>) -> Result<Json<Vec<$struct>>, String> {
            let result = $get_fn(pool).await.map_err(|e| format!("Database error: {}", e))?;
            Ok(Json(result))
        }

        pub mod $name {
            use super::*;
            pub async fn create(
                State(pool): State<Pool>,
                Json(item): Json<$new_struct>,
            ) -> Result<Json<Value>, String> {
                let result = $create_fn(pool, item).await.map_err(|e| format!("Database error: {}", e))?;
                Ok(Json(json!({ "rows_affected": result })))
            }

            #[derive(Debug, Deserialize)]
            pub struct DeleteParams {
                pub $key1: i32,
                pub $key2: i32,
                pub $key3: i32,
            }

            pub async fn delete(
                State(pool): State<Pool>,
                Query(params): Query<DeleteParams>,
            ) -> Result<Json<Value>, (StatusCode, String)> {
                let result = $delete_fn(pool, params.$key1, params.$key2, params.$key3).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
                if result == 0 {
                    return Err((StatusCode::NOT_FOUND, "Item not found".to_string()));
                }
                Ok(Json(json!({ "rows_affected": result })))
            }
        }
    };
}

// Plomba only has create and delete
macro_rules! crud_handlers_plomba {
    ($name:ident, $struct:ident, $new_struct:ident, $get_fn:ident, $create_fn:ident, $delete_fn:ident, $key1:ident, $key2:ident) => {
        pub async fn $name(State(pool): State<Pool>) -> Result<Json<Vec<$struct>>, String> {
            let result = $get_fn(pool).await.map_err(|e| format!("Database error: {}", e))?;
            Ok(Json(result))
        }

        pub mod $name {
            use super::*;
            pub async fn create(
                State(pool): State<Pool>,
                Json(item): Json<$new_struct>,
            ) -> Result<Json<Value>, String> {
                let result = $create_fn(pool, item).await.map_err(|e| format!("Database error: {}", e))?;
                Ok(Json(json!({ "rows_affected": result })))
            }

            #[derive(Debug, Deserialize)]
            pub struct DeleteParams {
                pub $key1: i32,
                pub $key2: i32,
            }

            pub async fn delete(
                State(pool): State<Pool>,
                Query(params): Query<DeleteParams>,
            ) -> Result<Json<Value>, (StatusCode, String)> {
                let result = $delete_fn(pool, params.$key1, params.$key2).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
                if result == 0 {
                    return Err((StatusCode::NOT_FOUND, "Item not found".to_string()));
                }
                Ok(Json(json!({ "rows_affected": result })))
            }
        }
    };
}

crud_handlers!(
    kraj_handler,
    Kraj,
    NewKraj,
    get_kraj,
    create_kraj,
    update_kraj,
    delete_kraj
);
crud_handlers!(
    okres_handler,
    Okres,
    NewOkres,
    get_okres,
    create_okres,
    update_okres,
    delete_okres
);
crud_handlers!(
    obec_handler,
    Obec,
    NewObec,
    get_obec,
    create_obec,
    update_obec,
    delete_obec
);
crud_handlers!(
    katastralni_uzemi_handler,
    KatastralniUzemi,
    NewKatastralniUzemi,
    get_katastralni_uzemi,
    create_katastralni_uzemi,
    update_katastralni_uzemi,
    delete_katastralni_uzemi
);
crud_handlers!(
    bpej_handler,
    Bpej,
    NewBpej,
    get_bpej,
    create_bpej,
    update_bpej,
    delete_bpej
);
crud_handlers!(
    typ_rizeni_handler,
    TypRizeni,
    NewTypRizeni,
    get_typ_rizeni,
    create_typ_rizeni,
    update_typ_rizeni,
    delete_typ_rizeni
);
crud_handlers!(
    typ_operace_handler,
    TypOperace,
    NewTypOperace,
    get_typ_operace,
    create_typ_operace,
    update_typ_operace,
    delete_typ_operace
);
crud_handlers!(
    typ_ucastnika_handler,
    TypUcastnika,
    NewTypUcastnika,
    get_typ_ucastnika,
    create_typ_ucastnika,
    update_typ_ucastnika,
    delete_typ_ucastnika
);
crud_handlers!(
    ucastnik_rizeni_handler,
    UcastnikRizeni,
    NewUcastnikRizeni,
    get_ucastnik_rizeni,
    create_ucastnik_rizeni,
    update_ucastnik_rizeni,
    delete_ucastnik_rizeni
);
crud_handlers!(
    list_vlastnictvi_handler,
    ListVlastnictvi,
    NewListVlastnictvi,
    get_list_vlastnictvi,
    create_list_vlastnictvi,
    update_list_vlastnictvi,
    delete_list_vlastnictvi
);
crud_handlers!(
    parcela_row_handler,
    ParcelaRow,
    NewParcelaRow,
    get_parcela_row,
    create_parcela_row,
    update_parcela_row,
    delete_parcela_row
);
crud_handlers!(
    rizeni_handler,
    Rizeni,
    NewRizeni,
    get_rizeni,
    create_rizeni,
    update_rizeni,
    delete_rizeni
);

crud_handlers!(
    majitel_handler,
    Majitel,
    NewMajitel,
    get_majitel,
    create_majitel,
    update_majitel,
    delete_majitel
);

crud_handlers_composite_2!(
    vlastnictvi_handler,
    Vlastnictvi,
    NewVlastnictvi,
    get_vlastnictvi,
    create_vlastnictvi,
    update_vlastnictvi,
    delete_vlastnictvi,
    parcela_id,
    majitel_id
);
crud_handlers_composite_2!(
    bremeno_parcela_parcela_handler,
    BremenoParcelaParcela,
    NewBremenoParcelaParcela,
    get_bremeno_parcela_parcela,
    create_bremeno_parcela_parcela,
    update_bremeno_parcela_parcela,
    delete_bremeno_parcela_parcela,
    parcela_id,
    parcela_povinna_id
);
crud_handlers_composite_2!(
    bremeno_parcela_majitel_handler,
    BremenoParcelaMajitel,
    NewBremenoParcelaMajitel,
    get_bremeno_parcela_majitel,
    create_bremeno_parcela_majitel,
    update_bremeno_parcela_majitel,
    delete_bremeno_parcela_majitel,
    parcela_id,
    majitel_povinny_id
);
crud_handlers_composite_2!(
    rizeni_operace_row_handler,
    RizeniOperaceRow,
    NewRizeniOperaceRow,
    get_rizeni_operace_row,
    create_rizeni_operace_row,
    update_rizeni_operace_row,
    delete_rizeni_operace_row,
    rizeni_id,
    typ_operace_id
);

crud_handlers_plomba!(
    plomba_handler,
    Plomba,
    NewPlomba,
    get_plomba,
    create_plomba,
    delete_plomba,
    rizeni_id,
    parcela_id
);

crud_handlers_composite_3!(
    ucast_handler,
    Ucast,
    NewUcast,
    get_ucast,
    create_ucast,
    delete_ucast,
    rizeni_id,
    ucastnik_rizeni_id,
    typ_ucastnika_id
);
