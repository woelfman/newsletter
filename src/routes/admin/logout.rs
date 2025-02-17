use axum::response::{IntoResponse, Redirect};
use axum_flash::Flash;
use reqwest::StatusCode;

use crate::{session_state::TypedSession, utils::e500};

pub async fn log_out(
    flash: Flash,
    session: TypedSession,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match session.get_user_id().await {
        Ok(user_id) => {
            if user_id.is_none() {
                return Ok(Redirect::to("/login").into_response());
            }
        }
        Err(e) => return Err(e500(e).into_response()),
    }
    session
        .log_out()
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;
    Ok((
        flash.info("You have successfully logged out."),
        Redirect::to("/login"),
    )
        .into_response())
}
