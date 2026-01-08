use anyhow::Result;
use axum::{
    middleware,
    routing::get,
    Extension, Router,
};
use clap::Parser;
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use katastr_server::{
    AppState, require_auth_cookie, track_latency,
    bpej_handler, bremeno_parcela_majitel_handler, bremeno_parcela_parcela_handler,
    get_authenticate, get_health, get_lv_data, get_parceala_data,
    get_spravni_rizeni, katastralni_uzemi_handler, kraj_handler, list_vlastnictvi_handler,
    majitel_handler, obec_handler, okres_handler, parcela_row_handler, plomba_handler,
    rizeni_handler, rizeni_operace_row_handler, typ_operace_handler, typ_rizeni_handler,
    typ_ucastnika_handler, ucast_handler, ucastnik_rizeni_handler, vlastnictvi_handler,
};
use tokio_postgres::NoTls;
use tower_http::compression::CompressionLayer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    password: String,

    #[arg(long, default_value_t = false)]
    no_print: bool,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut cfg = Config::new();
    cfg.user = Some("postgres".to_string());
    cfg.dbname = Some("postgres".to_string());
    cfg.host = Some("127.0.0.1".to_string());
    cfg.port = Some(5432);
    cfg.password = Some("heslo".to_string());
    cfg.pool = Some(deadpool_postgres::PoolConfig::new(100)); // Increase pool size
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    let password;
    match args.password.is_empty() {
        true => {
            // Default password hash (cost 12)
            password = "$2b$12$rgOkHM0IWEmHYTidLt2WmeQANUGlG1wJxwSeoFX/XPltU/8okgKW6".to_string();
        }
        false => {
            // User provided password: use DEFAULT_COST (12)
            password = bcrypt::hash(args.password, bcrypt::DEFAULT_COST)?;
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
            let s = state.clone();
            move |req, next| {
                let s = s.clone();
                async move { track_latency(s, req, next).await }
            }
        }));

            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

            if !args.no_print {

                println!("Server running on http://0.0.0.0:3000");

                println!("Press 'q' then Enter to stop");

            }

        

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

        

    