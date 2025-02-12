use std::sync::Arc;

use email_client::EmailClient;
use sqlx::PgPool;

pub mod configuration;
pub mod domain;
pub mod email_client;
pub mod routes;
pub mod startup;
pub mod telemetry;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
    email_client: Arc<EmailClient>,
    base_url: String,
}
