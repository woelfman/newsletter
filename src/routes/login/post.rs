use axum::{extract::State, response::Redirect, Form};
use axum_flash::Flash;
use secrecy::SecretString;

use crate::{
    authentication::{validate_credentials, Credentials},
    routes::error_chain_fmt,
    session_state::TypedSession,
    AppState,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: SecretString,
}

#[tracing::instrument(
    skip(state, form, flash, session),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    flash: Flash,
    session: TypedSession,
    Form(form): Form<FormData>,
) -> Result<Redirect, (Flash, Redirect)> {
    let credentials = Credentials {
        username: form.username,
        password: form.password,
    };
    tracing::Span::current().record("username", tracing::field::display(&credentials.username));
    match validate_credentials(credentials, &state.db_pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", tracing::field::display(&user_id));
            if let Err(e) = session.renew().await {
                return Err(login_redirect(flash, LoginError::UnexpectedError(e)));
            }

            if let Err(e) = session.insert_user_id(user_id).await {
                return Err(login_redirect(flash, LoginError::UnexpectedError(e)));
            }
            Ok(Redirect::to("/admin/dashboard"))
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
            Err((flash.error(e.to_string()), Redirect::to("/login")))
        }
    }
}

// Redirect to the login page with an error message.
fn login_redirect(flash: Flash, e: LoginError) -> (Flash, Redirect) {
    (flash.error(e.to_string()), Redirect::to("/login"))
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
