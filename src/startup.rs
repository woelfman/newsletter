use std::sync::Arc;

use axum::{
    routing::{get, post},
    serve::Serve,
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{
    email_client::EmailClient,
    routes::{health_check, subscribe},
    AppState,
};

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let state = AppState {
        db_pool,
        email_client: Arc::new(email_client),
    };
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let server = axum::serve(listener, app);

    Ok(server)
}
