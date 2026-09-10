// responses
// 1. simple types ( String, &str)
// 2. Json Response
// 3. HTML Response
// 4. Custom Response type
// 5. Status code and headers
// 6. intoresponse trait
mod tests;

use axum::{
    Router,
    body::Body,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{Html, IntoResponse, Json, Redirect, Response},
    routing::get,
};
use serde::Serialize;
use std::net::SocketAddr;

//simple response types
async fn stc_str() -> &'static str {
    "Hello World!"
}

// ngembaliin owned string
async fn own_str() -> String {
    format!("Hello World on timestamp: {}", chrono_lite())
}

//helper chrono_lite()
fn chrono_lite() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// tuple status code
async fn w_status() -> (StatusCode, &'static str) {
    (StatusCode::CREATED, "Created Succeded")
}

// Json Response
#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
    active: bool,
}

async fn active_user() -> Json<User> {
    Json(User {
        id: 1,
        name: "paritiw wiw wiw".to_string(),
        email: "paritiw@piwpiw".to_string(),
        active: true,
    })
}

#[derive(Serialize)]
struct UserResp {
    users: Vec<User>,
    total: usize,
    page: u32,
}

async fn json_users() -> Json<UserResp> {
    let users = vec![
        User {
            id: 1,
            name: "a".to_string(),
            email: "mail@1".to_string(),
            active: false,
        },
        User {
            id: 2,
            name: "b".to_string(),
            email: "mail@2".to_string(),
            active: true,
        },
    ];
    let total = users.len();
    Json(UserResp {
        users,
        total,
        page: 1,
    })
}

// json demgan custom status code
async fn json_user_status() -> (StatusCode, Json<User>) {
    (
        StatusCode::CREATED,
        Json(User {
            id: 3,
            name: "New User".to_string(),
            email: "new@example.com".to_string(),
            active: true,
        }),
    )
}

// static html doc file
async fn static_html() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <h1>hello world></h1>
        </html>
        "#,
    )
}

/// Dynamic HTML
async fn dynamic_html() -> Html<String> {
    let items = vec!["Routing", "Extractors", "Responses", "Middleware"];
    let list_items: String = items
        .iter()
        .map(|item| format!("<li>{}</li>", item))
        .collect();

    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Axum Course Modules</title>
            <style>
                body {{ font-family: system-ui; padding: 20px; }}
                ul {{ list-style-type: none; padding: 0; }}
                li {{ 
                    padding: 10px 15px;
                    margin: 5px 0;
                    background: #f0f0f0;
                    border-radius: 5px;
                }}
            </style>
        </head>
        <body>
            <h1>Course Topics</h1>
            <ul>{}</ul>
        </body>
        </html>
        "#,
        list_items
    ))
}

// custom response dengan headers
async fn wheader() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("max-age=3600"),
    );
    headers.insert("X-K-Api_key", HeaderValue::from_static("kiwkiwprikitiw"));
    (headers, "Response with custom headers")
}

// full resp Status + headers + body
async fn full_resp() -> (StatusCode, HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    headers.insert("X-Req_id", HeaderValue::from_static("183810128"));
    (StatusCode::OK, headers, "full controll response")
}

// redirect
async fn permanent_red() -> Redirect {
    Redirect::permanent("/permanent-closed")
}
async fn temp_red() -> Redirect {
    Redirect::temporary("/temp-closed")
}
async fn other_red() -> Redirect {
    Redirect::to("/succeded")
}
async fn new_location() -> &'static str {
    "redirected here"
}

// intoresponse trait
// custom response type implemen IntoResponse
struct CustomResp {
    msg: String,
    status: StatusCode,
}
impl IntoResponse for CustomResp {
    fn into_response(self) -> Response {
        let body = format!(
            r#"{{"msg": "{}", "status": "{}" }}"#,
            self.msg,
            self.status.as_u16()
        );
        Response::builder()
            .status(self.status)
            .header(header::CONTENT_TYPE, "Application/json")
            .body(Body::from(body))
            .unwrap_or_else(|_| {
                (StatusCode::INTERNAL_SERVER_ERROR, "err internal conflict").into_response()
            })
    }
}
async fn custom_resp() -> CustomResp {
    CustomResp {
        msg: "pushpush".to_string(),
        status: StatusCode::OK,
    }
}

// Api Response Wrapper untuk
// Json yang konsisten
// mirip pydantic python
#[derive(Serialize)]
struct ApiResp<T: Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T: Serialize> IntoResponse for ApiResp<T> {
    fn into_response(self) -> Response {
        let status = match self.success {
            true => StatusCode::OK,
            false => StatusCode::BAD_REQUEST,
        };
        (status, Json(self)).into_response()
    }
}

async fn api_success() -> ApiResp<User> {
    ApiResp {
        success: true,
        data: Some(User {
            id: 1,
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            active: true,
        }),
        error: None,
    }
}

async fn api_err() -> ApiResp<()> {
    ApiResp {
        success: false,
        data: None,
        error: Some("Something went wrong".to_string()),
    }
}

async fn maybe_error() -> Result<Json<User>, (StatusCode, String)> {
    let result = true;
    if result {
        Ok(Json(User {
            id: 1,
            name: "success User".to_string(),
            email: "1@email.com".to_string(),
            active: true,
        }))
    } else {
        Err((StatusCode::NOT_FOUND, "uSer not found".to_string()))
    }
}
#[tokio::main]
async fn main() {
    let app = Router::new()
        // Simple responses
        .route("/string", get(stc_str))
        .route("/owned", get(own_str))
        .route("/status", get(w_status))
        // JSON responses
        .route("/json/user", get(active_user))
        .route("/json/users", get(json_users))
        .route("/json/created", get(json_user_status))
        // HTML responses
        .route("/html", get(static_html))
        .route("/html/dynamic", get(dynamic_html))
        // Headers
        .route("/headers", get(wheader))
        .route("/full", get(full_resp))
        // Redirects
        .route("/redirect/permanent", get(permanent_red))
        .route("/redirect/temp", get(temp_red))
        .route("/redirect/other", get(other_red))
        .route("/new-location", get(new_location))
        .route("/temp-location", get(new_location))
        .route("/success", get(|| async { "Form submitted successfully!" }))
        // Custom responses
        .route("/custom", get(custom_resp))
        .route("/api/success", get(api_success))
        .route("/api/error", get(api_err))
        // Result type
        .route("/maybe-error", get(maybe_error));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("listening on {:?}", &addr);
    axum::serve(listener, app).await.expect("Server failed");
}
