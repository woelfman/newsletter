use axum::response::IntoResponse;
use reqwest::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ValidationError(String),
    DatabaseErrro(sqlx::Error),
    StoreTokenError(StoreTokenError),
    SendEmailError(reqwest::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ValidationError(e) => write!(f, "{e}"),
            Error::Reqwest(e) => write!(f, "An http client error was encountered: {e}"),
            Error::StoreTokenError(e) => write!(
                f,
                "A database error was encountered while trying to store a subscription token: {}",
                e
            ),
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::ValidationError(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Reqwest(value)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::StoreTokenError(value)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Error::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            Error::Reqwest(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            Error::StoreTokenError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
        };

        (status, message).into_response()
    }
}