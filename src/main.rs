use newsletter::{
    configuration::get_configuration,
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
    let address = format!("127.0.0.1:{}", configuration.application.port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    run(listener, connection_pool)?.await
}
