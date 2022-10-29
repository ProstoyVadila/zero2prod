use std::net::TcpListener;

use sqlx::{PgConnection, Connection};
use zero2prod::configuration::get_configuration;


#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    
    let client = reqwest::Client::new();
    let resp = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute requests.");

    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();

    let config = get_configuration().expect("Failed to read config.");
    let conn_string = config.database.connection_string();
    let mut conn = PgConnection::connect(&conn_string)
        .await
        .expect("Failed to connect to Postgres.");

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let resp = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    println!("resp status: {}", resp.status().as_str());
    assert_eq!(200, resp.status().as_u16());

    let saved = sqlx::query!("select email, name from subscriptions")
        .fetch_one(&mut conn)
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

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind a random port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    println!("spawning server at localhost:{}", port);
    format!("http://127.0.0.1:{}", port)
}