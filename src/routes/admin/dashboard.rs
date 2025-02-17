use anyhow::Context;
use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
};
use reqwest::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{session_state::TypedSession, AppState};

#[tracing::instrument(name = "Admin Dashboard", skip(state, session))]
pub async fn admin_dashboard(
    State(state): State<AppState>,
    session: TypedSession,
) -> Result<Html<String>, impl IntoResponse> {
    let username = if let Some(user_id) = session
        .get_user_id()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
    {
        get_username(user_id, &state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
    } else {
        return Err(Redirect::to("/login").into_response());
    };

    Ok(Html(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Admin dashboard</title>
</head>
<body>
    <p>Welcome {username}!</p>
    <p>Available actions:</p>
    <ol>
        <li><a href="/admin/password">Change password</a></li>
        <li>
            <form name="logoutForm" action="/admin/logout" method="post">
                <input type="submit" value="Logout">
            </form>
        </li>
    </ol>
</body>
</html>"#,
    )))
}

#[tracing::instrument(name = "Get username", skip(pool))]
pub async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE User_id = $1
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await
    .context("Failed to perform a query to retrieve a username.")?;
    Ok(row.username)
}
