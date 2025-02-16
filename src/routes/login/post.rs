use axum::{extract::State, response::Redirect, Form};
use hmac::{Hmac, Mac};
use secrecy::{ExposeSecret, SecretString};
use urlencoding::encode;

use crate::{
    authentication::{validate_credentials, Credentials},
    routes::error_chain_fmt,
    AppState,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: SecretString,
}

#[tracing::instrument(
    skip(state, form),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(State(state): State<AppState>, Form(form): Form<FormData>) -> Redirect {
    let credentials = Credentials {
        username: form.username,
        password: form.password,
    };
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
    match validate_credentials(credentials, &state.db_pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            Redirect::to("/")
        }
        Err(e) => {
            let e = match e {
                crate::authentication::AuthError::InvalidCredentials(_) => {
                    LoginError::AuthError(e.into())
                }
                crate::authentication::AuthError::UnexpectedError(_) => {
                    LoginError::UnexpectedError(e.into())
                }
            };
            let query_string = format!("error={}", encode(&e.to_string()));
            let hmac_tag = {
                let mac = Hmac::<sha2::Sha256>::new_from_slice(
                    state.hmac_secret.0.expose_secret().as_bytes(),
                )
                .unwrap();
                mac.finalize().into_bytes()
            };
            Redirect::to(&format!("/login?{}&tag={:x}", query_string, hmac_tag))
        }
    }
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
