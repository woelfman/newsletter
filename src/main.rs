use newsletter::{
    configuration::get_configuration,
    email_client::EmailClient,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsletter", "info", std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    // Build an `EmailClient` using `configuration`
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    let address = format!("127.0.0.1:{}", configuration.application.port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    run(listener, connection_pool, email_client)?.await
}
