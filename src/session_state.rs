use axum::{
    extract::FromRequestParts,
    http::{self, request::Parts},
};
use reqwest::StatusCode;
use tower_sessions::Session;
use uuid::Uuid;

#[derive(Clone)]
pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub async fn renew(&self) -> Result<(), anyhow::Error> {
        self.0.cycle_id().await.map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn insert_user_id(&self, user_id: Uuid) -> Result<(), anyhow::Error> {
        self.0
            .insert(Self::USER_ID_KEY, user_id)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn get_user_id(&self) -> Result<Option<Uuid>, anyhow::Error> {
        self.0
            .get(Self::USER_ID_KEY)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn log_out(self) -> Result<(), anyhow::Error> {
        self.0.delete().await.map_err(|e| anyhow::anyhow!(e))
    }
}

impl<S> FromRequestParts<S> for TypedSession
where
    S: Sync + Send,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let parts = parts.extensions.get::<TypedSession>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Can't extract session. Is `SessionManagerLayer` enabled?",
        ))?;

        Ok(parts)
    }
}
