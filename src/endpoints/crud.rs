use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::*;

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
