use std::fmt::Write;

use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_flash::IncomingFlashes;

use crate::{session_state::TypedSession, utils::e500};

pub async fn change_password_form(
    session: TypedSession,
    flash_messages: IncomingFlashes,
) -> Result<impl IntoResponse, Response> {
    if session.get_user_id().await.map_err(e500)?.is_none() {
        return Ok(Redirect::to("/login").into_response());
    }

    let mut msg_html = String::new();
    for (_level, content) in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", content).unwrap()
    }

    Ok(Html(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Change Password</title>
</head>
<body>
    {msg_html}
    <form action="/admin/password" method="post">
        <label>Current password
            <input
                type="password"
                placeholder="Enter current password"
                name="current_password"
            >
        </label>
        <br>
        <label>New password
            <input
                type="password"
                placeholder="Enter new password"
                name="new_password"
            >
        </label>
        <br>
        <label>Confirm new password
            <input
                type="password"
                placeholder="Type the new password again"
                name="new_password_check"
            >
        </label>
        <br>
        <button type="submit">Change password</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#,
    ))
    .into_response())
}
