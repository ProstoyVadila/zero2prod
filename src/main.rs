use sqlx::PgPool;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration");
    
    let pg_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)
        .expect("Failed to bind a port");

    run(listener, pg_pool)?.await
}
