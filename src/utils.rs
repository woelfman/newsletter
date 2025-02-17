use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub fn e500<E>(e: E) -> Response
where
    E: std::fmt::Display + std::fmt::Debug + 'static,
{
    tracing::error!("Internal server error: {:?}", e);
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
