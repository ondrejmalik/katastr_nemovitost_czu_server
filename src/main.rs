use anyhow::Result;
use axum::{Router, routing::get};
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use tower_http::compression::CompressionLayer;

use htmxtest::endpoints::{get_lv_data, get_parceala_data};

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> Result<()> {
    let mut cfg = Config::new();
    cfg.user = Some("postgres".to_string());
    cfg.dbname = Some("postgres".to_string());
    cfg.host = Some("127.0.0.1".to_string());
    cfg.port = Some(5432);
    cfg.password = Some("heslo".to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

    // Create the router and register the endpoint
    let app = Router::new()
        .route("/lv", get(get_lv_data))
        .route("/parcela", get(get_parceala_data))
        .with_state(pool)
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
