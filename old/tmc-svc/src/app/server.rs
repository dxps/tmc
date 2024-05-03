use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    process::exit,
    str::FromStr,
};

use clap::Parser;
use tmc_svc::{
    app::{
        config::get_config,
        db::{init_db_pool, ping_db},
        state::AppState,
    },
    web_api::init_router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    //
    let opt = Opt::parse();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!(
                "{},hyper=info,mio=info,sqlx=warn,tower_http=warn",
                opt.log_level
            ),
        )
    }
    tracing_subscriber::fmt::init();

    log::info!(
        "The server is starting up (pid {}) ... ",
        std::process::id()
    );

    let state = init_state().await;
    let router = init_router(state);

    let socket_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));
    let tcp_listener = TcpListener::bind(socket_addr).await.unwrap();

    log::info!("Listening for requests on http://{} ...", socket_addr);

    axum::serve(tcp_listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn init_state() -> AppState {
    let app_cfg = get_config().expect("Failed to load the app config.");

    let db_conn_pool = init_db_pool(&app_cfg)
        .await
        .expect("Failed to connect to database.");

    match ping_db(&db_conn_pool).await {
        true => log::info!(
            "Connected to the database (with {} conns).",
            db_conn_pool.size()
        ),
        false => {
            log::error!("Failed to ping the database. Exiting now.");
            exit(1);
        }
    }

    AppState::new(db_conn_pool)
}

async fn shutdown_signal() {
    //
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to init Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to init signal handler")
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    log::info!("Shutting down gracefully ...")
}

#[derive(Parser, Debug)]
#[clap(name = "server", about = "The server.")]
struct Opt {
    /// The HTTP listening address.
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// The HTTP listening port.
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// The logging level.
    #[clap(short = 'l', long = "log", default_value = "info")]
    log_level: String,

    /// The directory where assets (static) files are served from. <br/>
    /// These assets are fetched by requests using `/assets/*` path.
    #[clap(short = 's', long = "assets-dir", default_value = "../dist")]
    assets_dir: String,
}
