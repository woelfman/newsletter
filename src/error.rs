pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    StoreTokenError(sqlx::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::StoreTokenError(e) => write!(
                f,
                "A database error was encountered  while trying to store a subscription token: {}",
                e
            ),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::StoreTokenError(value)
    }
}