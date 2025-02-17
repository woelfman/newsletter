use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Form,
};
use axum_flash::Flash;
use secrecy::{ExposeSecret, SecretString};

use crate::{
    authentication::{validate_credentials, AuthError, Credentials},
    routes::admin::dashboard::get_username,
    session_state::TypedSession,
    utils::e500,
    AppState,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: SecretString,
    new_password: SecretString,
    new_password_check: SecretString,
}

pub async fn change_password(
    State(state): State<AppState>,
    session: TypedSession,
    flash: Flash,
    Form(form): Form<FormData>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let user_id = match session.get_user_id().await {
        Ok(user_id) => user_id,
        Err(e) => return Err(e500(e)),
    };
    if user_id.is_none() {
        return Ok(Redirect::to("/login").into_response());
    }
    let user_id = user_id.unwrap();

    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        return Err((
            flash.error("You entered two different new passwords - the field values must match."),
            Redirect::to("/admin/password"),
        )
            .into_response());
    }
    let username = get_username(user_id, &state.db_pool).await.map_err(e500)?;

    let credentials = Credentials {
        username,
        password: form.current_password,
    };
    if let Err(e) = validate_credentials(credentials, &state.db_pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => Ok((
                flash.error("The current password is incorrect."),
                Redirect::to("/admin/password"),
            )
                .into_response()),
            AuthError::UnexpectedError(_) => Err(e500(e).into_response()),
        };
    }
    crate::authentication::change_password(user_id, form.new_password, &state.db_pool)
        .await
        .map_err(e500)?;
    Ok((
        flash.error("Your password has been changed."),
        Redirect::to("/admin/password"),
    )
        .into_response())
}
