// middleware layer
//! Tower middleware integration in Axum:
//! - Built-in middleware (CORS, Compression, Timeout)
//! - Custom middleware with from_fn
//! - Route-specific layers

use std::time::{Duration, Instant};

use axum::{
    extract::Request, 
    http::{HeaderValue, Method, StatusCode, header}, 
    middleware::{self, Next}, 
    response::{IntoResponse, Response},
    Router,
    routing::{get}};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer
};
use tracing::Level;
use axum_test::TestServer;

async fn logging_middleware(
    req: Request,
    next: Next
) -> Response {
    let method = req.method().clone();
    let url = req.uri().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    tracing::info!(
        method = %method,
        url = %url,
        status = %&response.status().as_u16(),
        duration_ms = %start.elapsed().as_millis(),
        "Request completed"
        );
        response
}

async fn timing_middleware(
    req: Request,
    next: Next
) -> Response {
    let start = Instant::now();
    let mut response = next.run(req).await;

    response.headers_mut().insert(
        "X-Response-Time",
        HeaderValue::from_str(&format!("{}ms",
            start.elapsed().as_millis())).unwrap(),
    );
    response
}

async fn auth_middleware(
    req: Request,
    next: Next
) -> Result<Response, StatusCode > {
    let header = req
        .headers()
        .get("X-API-KEY")
        .and_then(|v| v.to_str().ok());

    match header {
        Some("secret-key") => Ok(
            next.run(req).await
        ),
        _ => Err(StatusCode::UNAUTHORIZED)
    }
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(
            [
                Method::GET,
                Method::POST,
                Method::PUT,
            ]
        )
        .allow_headers(
            [
                header::CONTENT_TYPE,
                header::AUTHORIZATION
            ]
        )
}

async fn index() -> &'static str {
    "welcome k axum learning module"
}

async fn _public() -> impl IntoResponse {
    axum::Json(serde_json::json!({
        "message" : "data public",
        "accessible" : "true"
    }))
}

async fn _protected() -> impl IntoResponse {
    axum::Json(serde_json::json!({
        "message" : "protected data",
        "authorized" : true
    }))
}

async fn _slow() -> &'static str {
    tokio::time::sleep(Duration::from_secs(2)).await;
    "slow operation done!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(
        Level::INFO
    ).init();

    let protected = Router::new()
        .route("/data", get(_protected))
        .route_layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        .route("/", get(index))
        .route("/public", get(_public))
        .route("/slow", get(_slow))
        .nest("/protected", protected)
        .layer(middleware::from_fn(timing_middleware))
        .layer(middleware::from_fn(logging_middleware))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors_layer())
                .layer(CompressionLayer::new()),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
