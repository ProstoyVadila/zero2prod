use zero2prod::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let localhost = "127.0.0.1:0";
    let listener = TcpListener::bind(&localhost)
        .expect("Failed to bind a random port");

    format!("{}:{}", localhost, listener.local_addr().unwrap().port());
    run(listener)?.await
}
