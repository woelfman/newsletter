use std::sync::Arc;

use axum::{
    routing::{get, post},
    serve::Serve,
    Router,
};
use secrecy::{ExposeSecret, SecretString};
use sqlx::{postgres::PgPoolOptions, PgPool};
use time::Duration;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::{fred::prelude::*, RedisStore};

use crate::{
    configuration::{DatabaseSettings, Settings},
    email_client::EmailClient,
    routes::{
        admin_dashboard, change_password, change_password_form, confirm, health_check, home,
        log_out, login, login_form, publish_newsletter, subscribe,
    },
    AppState,
};

pub struct Application {
    port: u16,
    server: Serve<TcpListener, Router, Router>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
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
            configuration.application.hmac_secret,
            configuration.redis_uri,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
    hmac_secret: SecretString,
    redis_uri: SecretString,
) -> Result<Serve<TcpListener, Router, Router>, anyhow::Error> {
    let key = axum_flash::Key::from(hmac_secret.expose_secret().as_bytes());
    let state = AppState {
        db_pool,
        email_client: Arc::new(email_client),
        base_url,
        hmac_secret: HmacSecret(hmac_secret),
        flash_config: axum_flash::Config::new(key),
    };

    let pool = Pool::new(
        Config::from_url(redis_uri.expose_secret())?,
        None,
        None,
        None,
        6,
    )
    .unwrap();
    let _redis_conn = pool.connect();
    pool.wait_for_connect().await?;
    let session_store = RedisStore::new(pool);
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));

    let app = Router::new()
        .route("/", get(home))
        .route("/admin/dashboard", get(admin_dashboard))
        .route("/admin/logout", post(log_out))
        .route("/admin/password", get(change_password_form))
        .route("/admin/password", post(change_password))
        .route("/health_check", get(health_check))
        .route("/login", get(login_form))
        .route("/login", post(login))
        .route("/newsletters", post(publish_newsletter))
        .route("/subscriptions", post(subscribe))
        .route("/subscriptions/confirm", get(confirm))
        .with_state(state)
        .layer(session_layer)
        .layer(TraceLayer::new_for_http());

    let server = axum::serve(listener, app);

    Ok(server)
}

#[derive(Clone)]
pub struct HmacSecret(pub SecretString);

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}
