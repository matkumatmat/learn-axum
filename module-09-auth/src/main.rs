// auth
// jwt authentication
// token generation
// auth middleware
// protected routes

use axum::{
    Json, Router,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Config
#[derive(Clone)]
struct AuthCfg {
    jwt_sec: String,
    jwt_exp_h: i64,
}

// Models
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    role: String,
}

#[derive(Deserialize)]
struct LoginReq {
    email: String,
    pwd: String,
}

#[derive(Serialize)]
struct LoginRes {
    token: String,
    exp_in: i64,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct RegisterReq {
    name: String,
    email: String,
    pwd: String,
}

#[derive(Debug, Clone)]
struct CurrentUser {
    id: String,
    role: String,
}

//password hashing
fn pwd_hash(
    pwd : &str
) -> String {
use argon2::{
    Argon2,
    PasswordHasher};

Argon2::default()
    .hash_password(pwd.as_bytes())
    .unwrap()
    .to_string()
}

#[allow(dead_code)]
fn pwd_verif(
    pwd: &str,
    hash: &str
) -> bool {
    use argon2::{
        Argon2,
        PasswordHash,
        PasswordVerifier};
    let x = PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(pwd.as_bytes(), &x)
        .is_ok()

}

// jwt
 fn token_create (
     cfg : &AuthCfg,
     user_id : &str,
     role : &str
 )-> Result<String, StatusCode> {
    let exp = Utc::now() + Duration::hours(
        cfg.jwt_exp_h
    );
    let claims = Claims {
        sub : user_id.to_string(),
        exp : exp.timestamp() as usize,
        role : role.to_string()
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.jwt_sec.as_bytes())
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
 }

fn token_verify(
    cfg: &AuthCfg,
    token : &str
) -> Result<Claims, StatusCode> {
    decode::<Claims> (
        token,
        &DecodingKey::from_secret(cfg.jwt_sec.as_bytes()
    ),
    &Validation::default(),
    )
        .map(|x| x.claims)
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

// HANDLERS

async fn register(
    Json(input) : Json<RegisterReq>
)-> impl IntoResponse {
    let x = pwd_hash(&input.pwd);
    Json(serde_json::json!({
        "msg" : "user registered",
        "email" : input.email
    }))
}


async fn login(
    State(cfg) : State<Arc<AuthCfg>>,
    Json(input) : Json<LoginReq>
) -> Result<Json<LoginRes>, StatusCode>{
    if input.email == "test@example.com" && input.pwd == "password123" {
        let token = token_create(
            &cfg,
            "user-1",
            "user"
        )?;
        
        Ok(Json(LoginRes{
            token,
            exp_in: cfg.jwt_exp_h * 3600
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}


async fn admin (
    axum::Extension(user): axum::Extension<CurrentUser>
)-> impl IntoResponse{
    if user.role != "admin" {
        return (StatusCode::FORBIDDEN, "access denied").into_response();
    }
    Json(serde_json::json!({
        "msg" : "admin areas",
        "user" : user.id
    })).into_response()
}

async fn protected(
    axum::Extension(user): axum::Extension<CurrentUser>
)-> impl IntoResponse{
    Json(serde_json::json!({
        "msg" : "access_granted",
        "user_id" : user.id,
        "role" : user.role
    }))
}

// auth middleware
async fn auth_middleware(
    State(cfg) : State<Arc<AuthCfg>>,
    mut req : Request,
    next : Next
) -> Result<Response, StatusCode> {
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    let token = header.ok_or(
        StatusCode::UNAUTHORIZED
    )?;
    let claims = token_verify(&cfg, token)?;
    let user = CurrentUser {
        id : claims.sub,
        role : claims.role
    };
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}


// main handler
#[tokio::main]
async fn main(){
    let cfg = Arc::new(
        AuthCfg {
            jwt_sec : "super-secret-key".to_string(),
            jwt_exp_h : 24
        }
    );
    let route_protected = Router::new()
        .route("/me",get(protected))
        .route("/admin", get(admin))
        .route_layer(
            middleware::from_fn_with_state(
                cfg.clone(),
                auth_middleware
        ));

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .nest("/protected", route_protected)
        .with_state(cfg);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("[INFO] server running on : http://localhost:3000");
    axum::serve(
        listener,
        app
    ).await.unwrap();
}
