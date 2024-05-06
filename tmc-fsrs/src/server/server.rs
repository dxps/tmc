#[cfg(feature = "server")]
use dioxus::dioxus_core::Element;

#[cfg(feature = "server")]
pub fn start(app_fn: fn() -> Element) {
    //
    use crate::{auth::User, server::connect_to_pgdb};
    use axum::routing::*;
    use axum_session::{SessionConfig, SessionLayer};
    use axum_session_auth::{AuthConfig, AuthSessionLayer};
    use axum_session_sqlx::{SessionPgPool, SessionPgSessionStore};
    use dioxus::prelude::*;
    use sqlx::PgPool;

    init_logging();

    tokio::runtime::Runtime::new().unwrap().block_on(async move {
        println!("Connecting to the database ...");
        let pg_pool = connect_to_pgdb().await;
        if pg_pool.is_err() {
            log::error!(
                "Failed to connect to database due to '{}'. Exiting now!",
                pg_pool.unwrap_err()
            );
            return;
        }
        let pg_pool = pg_pool.unwrap();
        log::info!("Connected to the database.");

        // This defaults as normal cookies.
        let session_config = SessionConfig::default().with_table_name("users_sessions");
        let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
        let session_store = SessionPgSessionStore::new(Some(pg_pool.clone().into()), session_config)
            .await
            .unwrap();

        User::create_user_tables(&pg_pool).await;

        let web_api_router = Router::new()
            // Server side render the application, serve static assets, and register server functions.
            .serve_dioxus_application(ServeConfig::builder().build(), move || VirtualDom::new(app_fn))
            .await
            .layer(AuthSessionLayer::<User, i64, SessionPgPool, PgPool>::new(Some(pg_pool)).with_config(auth_config))
            .layer(SessionLayer::new(session_store));

        // Start it.
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        axum::serve(listener, web_api_router.into_make_service()).await.unwrap();
    });
}

#[cfg(feature = "server")]
fn init_logging() {
    use log::LevelFilter;

    simple_logger::SimpleLogger::new()
        .with_module_level("sqlx", LevelFilter::Info)
        .with_module_level("tungstenite", LevelFilter::Info)
        .with_module_level("tokio_tungstenite", LevelFilter::Info)
        .with_module_level("axum_session", LevelFilter::Info)
        .with_module_level("axum_session_auth", LevelFilter::Warn)
        .with_module_level("dioxus_core", LevelFilter::Warn)
        .with_module_level("dioxus_signals", LevelFilter::Info)
        .with_module_level("tracing", LevelFilter::Warn)
        .init()
        .unwrap();
}