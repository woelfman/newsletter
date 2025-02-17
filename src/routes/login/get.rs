use std::fmt::Write;

use axum::response::{Html, IntoResponse};
use axum_flash::{IncomingFlashes, Level};

pub async fn login_form(flash_messages: IncomingFlashes) -> (IncomingFlashes, impl IntoResponse) {
    let mut error_html = String::new();
    for (_level, content) in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", content).unwrap();
    }
    (
        flash_messages,
        Html(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
</head>
<body>
    {error_html}
    <form action="/login" method="post">
        <label>Username
            <input
                type="text"
                placeholder="Enter Username"
                name="username"
            >
        </label>
        <label>Password
            <input
                type="password"
                placeholder="Enter Password"
                name="password"
            >
        </label>
        <button type="submit">Login</button>
    </form>
</body>
</html>"#,
        ))
        .into_response(),
    )
}
