use std::sync::Arc;

use axum::extract::FromRef;
use email_client::EmailClient;
use sqlx::PgPool;
use startup::HmacSecret;

pub mod authentication;
pub mod configuration;
pub mod domain;
pub mod email_client;
pub mod routes;
pub mod session_state;
pub mod startup;
pub mod telemetry;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    db_pool: PgPool,
    email_client: Arc<EmailClient>,
    base_url: String,
    hmac_secret: HmacSecret,
    flash_config: axum_flash::Config,
}

impl FromRef<AppState> for axum_flash::Config {
    fn from_ref(state: &AppState) -> axum_flash::Config {
        state.flash_config.clone()
    }
}
