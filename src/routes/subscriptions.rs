use axum::{extract::State, http::StatusCode, Form};
use chrono::Utc;
use sqlx::PgPool;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::{
    domain::{NewSubscriber, SubscriberEmail, SubscriberName},
    AppState,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(form: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(form.name)?;
        let email = SubscriberEmail::parse(form.email)?;
        Ok(NewSubscriber { email, name })
    }
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(state, form),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(State(state): State<AppState>, Form(form): Form<FormData>) -> StatusCode {
    let new_subscriber = match form.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return StatusCode::BAD_REQUEST,
    };
    match insert_subscriber(&state.db_pool, &new_subscriber).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// Returns `true if the input satisfies all our validation constraints on
/// subscriber names, `false` otherwise.
pub fn is_valid_name(s: &str) -> bool {
    let is_empty_or_whitespace = s.trim().is_empty();

    let is_too_long = s.graphemes(true).count() > 256;

    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(pool, new_subscriber)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
