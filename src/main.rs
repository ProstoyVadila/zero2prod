use sqlx::PgPool;
use env_logger::Env;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let pg_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)
        .expect("Failed to bind a port");

    run(listener, pg_pool)?.await
}
