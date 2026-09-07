
use axum::http::StatusCode;
use axum::{Router,routing::{get,post}};

pub async fn hello_world() -> &'static str {
    "hello world"
}

pub async fn health_check() -> &'static str {
    "200 OK"
}

pub async fn hello_axum() -> String {
    format!("welcome to axum {}", env!("CARGO_PKG_VERSION"))
}

pub async fn hello_status() -> (StatusCode, & 'static str) {
    (StatusCode::CREATED, "resources created")
}

pub async fn conditional_response() -> (StatusCode, &'static str) {
    let is_working = true;
    if is_working {
        (StatusCode::OK, "everything ok")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "error service brother")
    }
}

pub async  fn echo(body: String) -> String {
    format!("you sent : {}", body)
}

#[tokio::main]
pub async fn main(){
    let apps = Router::new()
        .route("/", get(hello_world))
        .route("/health",get(health_check))
        .route("/hello",get(hello_axum))
        .route("/created", get(hello_status))
        .route("/echo",post(echo));
    let listener= tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("error starting server");
    axum::serve(listener, apps)
        .await
        .expect("error starting apps");
}