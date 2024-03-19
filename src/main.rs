//!main.rs
use std::net::TcpListener;
use actix_web::web::get;
use sqlx::PgPool;
use tracing::{subscriber::{self, set_global_default}, Subscriber};
use tracing_log::LogTracer;
use Zero2pro::{configuration::get_configuration, startup::run};
use Zero2pro::telemetry::{get_subscriber,init_subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, FmtSubscriber, Registry};
use secrecy::ExposeSecret;


#[tokio::main]
async fn main() ->std::io::Result<()>{
    let subscriber = get_subscriber(
        "Zeropro".into(), 
        "info".into(),
        std::io::stdout
    );
    init_subscriber(subscriber);
    

    let configuration = get_configuration()
        .expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}