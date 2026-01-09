use anyhow::Result;
use axum::{Extension, Router, middleware, routing::get};
use clap::Parser;
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use katastr_server::{
    AppState, bpej_handler, bremeno_parcela_majitel_handler, bremeno_parcela_parcela_handler,
    get_authenticate, get_health, get_lv_data, get_parceala_data, get_spravni_rizeni,
    katastralni_uzemi_handler, kraj_handler, list_vlastnictvi_handler, majitel_handler,
    obec_handler, okres_handler, parcela_row_handler, plomba_handler, require_auth_cookie,
    rizeni_handler, rizeni_operace_row_handler, track_latency, typ_operace_handler,
    typ_rizeni_handler, typ_ucastnika_handler, ucast_handler, ucastnik_rizeni_handler,
    vlastnictvi_handler,
};
use mimalloc::MiMalloc;
use std::net::SocketAddr;
use tokio_postgres::NoTls;
use tower_http::compression::CompressionLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
#[global_allocator]
static GLOBAL_ALLOC: MiMalloc = MiMalloc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = false)]
    no_print: bool,

    #[arg(short, long, default_value = "")]
    server_password: String,

    #[arg(long, default_value_t = 3000)]
    server_port: u16,

    #[arg(long, default_value = "postgres")]
    db_user: String,

    #[arg(long, default_value = "postgres")]
    db_name: String,

    #[arg(long, default_value = "127.0.0.1")]
    db_host: String,

    #[arg(long, default_value = "heslo")]
    db_password: String,
    #[arg(long, default_value_t = 5432)]
    db_port: u16,
}
fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .worker_threads(num_cpus::get())
        .build()
        .unwrap();

    rt.block_on(async {
        if let Err(e) = run().await {
            eprintln!("Application error: {:?}", e);
            std::process::exit(1);
        }
    });
}
async fn run() -> Result<()> {
    let args = Args::parse();

    if args.no_print {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new(
                "katastr_server=off,tower_http=off,hyper=off",
            ))
            .with_target(false)
            .init();
    } else {
        tracing_subscriber::fmt().init();
    }

    info!("tracing initialized");

    let mut cfg = Config::new();
    cfg.user = Some(args.db_user.clone());
    cfg.dbname = Some(args.db_name.clone());
    cfg.host = Some(args.db_host.clone());
    cfg.port = Some(args.db_port);
    cfg.password = Some(args.db_password.to_string());
    cfg.pool = Some(deadpool_postgres::PoolConfig::new(num_cpus::get() / 2));
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    let password;
    match args.server_password.is_empty() {
        true => {
            // Default password hash (cost 12)
            password = "$2b$12$rgOkHM0IWEmHYTidLt2WmeQANUGlG1wJxwSeoFX/XPltU/8okgKW6".to_string();
        }
        false => {
            // User provided password: use DEFAULT_COST (12)
            password = bcrypt::hash(args.server_password, bcrypt::DEFAULT_COST)?;
        }
    }

    let state = AppState {
        password: password.to_string(),
        no_print: args.no_print,
        sessions: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
    };
    let app = Router::new()
        .route("/health", get(get_health))
        .route("/auth", get(get_authenticate))
        .route("/lv", get(get_lv_data))
        .route("/parcela", get(get_parceala_data))
        .route("/spravni_rizeni", get(get_spravni_rizeni))
        .route(
            "/majitel",
            get(majitel_handler)
                .post(majitel_handler::create)
                .put(majitel_handler::update)
                .delete(majitel_handler::delete),
        )
        .route(
            "/kraj",
            get(kraj_handler)
                .post(kraj_handler::create)
                .put(kraj_handler::update)
                .delete(kraj_handler::delete),
        )
        .route(
            "/okres",
            get(okres_handler)
                .post(okres_handler::create)
                .put(okres_handler::update)
                .delete(okres_handler::delete),
        )
        .route(
            "/obec",
            get(obec_handler)
                .post(obec_handler::create)
                .put(obec_handler::update)
                .delete(obec_handler::delete),
        )
        .route(
            "/katastralni_uzemi",
            get(katastralni_uzemi_handler)
                .post(katastralni_uzemi_handler::create)
                .put(katastralni_uzemi_handler::update)
                .delete(katastralni_uzemi_handler::delete),
        )
        .route(
            "/bpej",
            get(bpej_handler)
                .post(bpej_handler::create)
                .put(bpej_handler::update)
                .delete(bpej_handler::delete),
        )
        .route(
            "/typ_rizeni",
            get(typ_rizeni_handler)
                .post(typ_rizeni_handler::create)
                .put(typ_rizeni_handler::update)
                .delete(typ_rizeni_handler::delete),
        )
        .route(
            "/typ_operace",
            get(typ_operace_handler)
                .post(typ_operace_handler::create)
                .put(typ_operace_handler::update)
                .delete(typ_operace_handler::delete),
        )
        .route(
            "/typ_ucastnika",
            get(typ_ucastnika_handler)
                .post(typ_ucastnika_handler::create)
                .put(typ_ucastnika_handler::update)
                .delete(typ_ucastnika_handler::delete),
        )
        .route(
            "/ucastnik_rizeni",
            get(ucastnik_rizeni_handler)
                .post(ucastnik_rizeni_handler::create)
                .put(ucastnik_rizeni_handler::update)
                .delete(ucastnik_rizeni_handler::delete),
        )
        .route(
            "/list_vlastnictvi",
            get(list_vlastnictvi_handler)
                .post(list_vlastnictvi_handler::create)
                .put(list_vlastnictvi_handler::update)
                .delete(list_vlastnictvi_handler::delete),
        )
        .route(
            "/parcela_row",
            get(parcela_row_handler)
                .post(parcela_row_handler::create)
                .put(parcela_row_handler::update)
                .delete(parcela_row_handler::delete),
        )
        .route(
            "/rizeni",
            get(rizeni_handler)
                .post(rizeni_handler::create)
                .put(rizeni_handler::update)
                .delete(rizeni_handler::delete),
        )
        .route(
            "/vlastnictvi",
            get(vlastnictvi_handler)
                .post(vlastnictvi_handler::create)
                .put(vlastnictvi_handler::update)
                .delete(vlastnictvi_handler::delete),
        )
        .route(
            "/bremeno_parcela_parcela",
            get(bremeno_parcela_parcela_handler)
                .post(bremeno_parcela_parcela_handler::create)
                .put(bremeno_parcela_parcela_handler::update)
                .delete(bremeno_parcela_parcela_handler::delete),
        )
        .route(
            "/bremeno_parcela_majitel",
            get(bremeno_parcela_majitel_handler)
                .post(bremeno_parcela_majitel_handler::create)
                .put(bremeno_parcela_majitel_handler::update)
                .delete(bremeno_parcela_majitel_handler::delete),
        )
        .route(
            "/rizeni_operace",
            get(rizeni_operace_row_handler)
                .post(rizeni_operace_row_handler::create)
                .put(rizeni_operace_row_handler::update)
                .delete(rizeni_operace_row_handler::delete),
        )
        .route(
            "/plomba",
            get(plomba_handler)
                .post(plomba_handler::create)
                .delete(plomba_handler::delete),
        )
        .route(
            "/ucast",
            get(ucast_handler)
                .post(ucast_handler::create)
                .delete(ucast_handler::delete),
        )
        .with_state(pool)
        .layer(Extension(state.clone()))
        .layer(middleware::from_fn({
            let s = state.clone();
            move |req, next| {
                let s = s.clone();
                async move { require_auth_cookie(s, req, next).await }
            }
        }))
        .layer(CompressionLayer::new())
        .layer(middleware::from_fn({
            move |req, next| async move { track_latency(req, next).await }
        }));

    let ip = SocketAddr::from(([0, 0, 0, 0], args.server_port));

    let listener = match tokio::net::TcpListener::bind(ip).await {
        Ok(l) => {
            info!("{}", ip);
            l
        }
        Err(e) => {
            tracing::error!(%e, "Failed to bind to {}", ip);
            return Err(anyhow::anyhow!(e));
        }
    };

    info!("Server running on http://{}", ip);
    info!("Press 'q' then Enter to stop");

    axum::serve(listener, app)
        .with_graceful_shutdown(wait_for_q())
        .await?;

    Ok(())
}

async fn wait_for_q() {
    use tokio::io::{AsyncBufReadExt, BufReader};

    let stdin = tokio::io::stdin();

    let mut reader = BufReader::new(stdin);

    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line).await {
            Ok(0) => break, // EOF

            Ok(_) => {
                if line.trim() == "q" {
                    break;
                }
            }

            Err(_) => break,
        }
    }
}
