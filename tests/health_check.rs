use std::net::TcpListener;

use sqlx::{PgPool, Executor, PgConnection, Connection};
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};

pub struct TestApp {
    pub address: String,
    pub pg_pool: PgPool,
}


#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;
    
    let client = reqwest::Client::new();
    let resp = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute requests.");

    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let resp = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    println!("resp status: {}", resp.status().as_str());
    assert_eq!(200, resp.status().as_u16());

    let saved = sqlx::query!("select email, name from subscriptions")
        .fetch_one(&app.pg_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

// #[actix_rt::test]
// async fn subcribe_returns_a_400_when_data_is_missing() {
//     let app_address = spawn_app();
//     let client = reqwest::Client::new();
//     let test_cases = vec![
//         ("name=le%20guin", "missing the email"),
//         ("email=ursula_le_guin%40gmail.com", "missing the name"),
//         ("", "missing both name and email")
//     ];
//     for (invalid_body, error_message) in test_cases {
//         let resp = client
//             .post(&format!("{}/subscriptions", &app_address))
//             .header("Content-Type", "application/x-www-form-urlencoded")
//             .body(invalid_body)
//             .send()
//             .await
//             .expect("Failed to execute request.");
//         assert_eq!(
//             400,
//             resp.status().as_u16(),
//             "The API did not fail with 400 Bad Request when payload was {}",
//             error_message
//         );
//     }
// }

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind a random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut config = get_configuration().expect("Failed to load config");
    config.database.db_name = Uuid::new_v4().to_string();

    let pg_pool = configure_db(&config.database).await;

    let server = zero2prod::startup::run(listener, pg_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        pg_pool,
    }
}

async fn configure_db(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.db_name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}