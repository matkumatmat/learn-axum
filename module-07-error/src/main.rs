// axum error handling
//! Proper error handling in Axum:
//! - Custom error types with thiserror
//! - IntoResponse for errors
//! - Result-based handlers
//! - Error recovery patterns


use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json,
    Router
};
use serde::{Serialize};
use thiserror::Error;

#[derive(Error,Debug)]
#[allow(dead_code)]
enum AppErr{
    #[error("User not Found Error : {0}")]
    UserNotFound(u64),

    #[error("Invalid Input Error : {0}")]
    InvalidInput(String),

    #[error("Database Error : {0}")]
    DatabaseError(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal Service Error ")]
    InternalError,
}

#[derive(Debug, Serialize)]
struct ErrResponse{
    error: String,
    code : u16
}

impl IntoResponse for AppErr {
    fn into_response(self) -> Response {
        let (status, message ) = match &self{
            AppErr::UserNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppErr::InvalidInput(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppErr::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppErr::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppErr::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        let body = ErrResponse {
            error: message,
            code : status.as_u16(),
        };
    (status, Json(body)).into_response()

    }
}

#[derive(Serialize)]
struct User {
    id : u64,
    name : String
}
async fn get_user(
    Path(id) : Path<u64>
) -> Result<Json<User>, AppErr> {
    match id {
        1 => Ok(
            Json(
                User {
                    id: 1,
                    name : "alucrot".to_string()
                }
            )
        ),
        2 => Ok(
            Json(
                User {
                    id : 2,
                    name : "nana".to_string()
                }
            )
        ),
        _ => Err(AppErr::UserNotFound(id))
    }
}

async fn validate_input(
    Path(val) : Path<String> 
) -> Result<String, AppErr> {
    if val.len() < 3 {
        return Err(AppErr::InvalidInput(
            "Value must be at least 3 chars".to_string()
        ));
    }
    Ok(format!("valid input : {}", val))
}

async fn _protected() -> Result<&'static str, AppErr> {
    let is_auth = true;
    if !is_auth {
        return Err(AppErr::Unauthorized);
    }
    Ok("Secret bruh")
}

async fn _db_ops() -> Result<&'static str, AppErr> {
    // Simulated database error
    Err(AppErr::DatabaseError("Connection timeout".to_string()))
}

async fn _complex(
    Path(id) : Path<u64>
) -> Result<Json<User>, AppErr> {
    // Use ? operator for early returns
    let user = _find(id)?;
    _validate(&user)?;
    Ok(Json(user))
}
fn _find(
    id: u64
) -> Result<User, AppErr> {
    if id == 0 {
        Err(AppErr::InvalidInput("id cannot be zero".to_string()))
    } else if id > 100 {
        Err(AppErr::UserNotFound(id))
    } else {
        Ok(User {
            id,
            name: format!("User{}", id),
        })
    }
}

fn _validate(
    u:&User
) -> Result<(), AppErr> {
    if u.name.is_empty() {
        Err(AppErr::InvalidInput("name cannot be blank".to_string()))
    } else {
        Ok(())
    }
}






#[tokio::main]
async fn main(){
    let app: Router<()> = Router::new()
        .route("/users/{id}", get(get_user))
        .route("/validate/{val}", get(validate_input))
        .route("/protected", get(_protected))
        .route("/database", get(_db_ops))
        .route("/complex/{id}", get(_complex));
    let addr = "0.0.0.0:3000".to_string();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!(">> listening on {:?}", &addr);
    axum::serve(listener, app).await.unwrap();
}

