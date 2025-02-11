use axum::http::StatusCode;

#[tracing::instrument(name = "Health check")]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
