use std::net::TcpListener;
use secrecy::ExposeSecret;

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool};
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{init_subscriber, get_subscriber};

// #[actix_web::main] // or 
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres.");
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, pool)?.await
}
