use std::sync::Arc;

use email_client::EmailClient;
use sqlx::PgPool;

pub mod configuration;
pub mod domain;
pub mod email_client;
// pub mod error;
pub mod routes;
pub mod startup;
pub mod telemetry;

// pub use error::{Error, Result};

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
    email_client: Arc<EmailClient>,
    base_url: String,
}
