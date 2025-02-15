use std::sync::Arc;

use axum::{
    routing::{get, post},
    serve::Serve,
    Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{
    configuration::{DatabaseSettings, Settings},
    email_client::EmailClient,
    routes::{confirm, health_check, publish_newsletter, subscribe},
    AppState,
};

pub struct Application {
    port: u16,
    server: Serve<TcpListener, Router, Router>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool =
            PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address).await?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url.clone(),
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let state = AppState {
        db_pool,
        email_client: Arc::new(email_client),
        base_url,
    };
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .route("/subscriptions/confirm", get(confirm))
        .route("/newsletters", post(publish_newsletter))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let server = axum::serve(listener, app);

    Ok(server)
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}
