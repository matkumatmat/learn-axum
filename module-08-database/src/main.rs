// database migrations
//! SQLx with PostgreSQL in Axum:
//! - Connection pooling
//! - CRUD operations
//! - Query macros
//! - Migrations


use sqlx::{PgPool, postgres::PgPoolOptions, types::chrono};
use uuid::Uuid;
use serde::{Deserialize,Serialize};
use axum::{
    extract::{Path,State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json,
    Router
};

// models
#[derive(Debug, Serialize, sqlx::FromRow)]
struct User {
    id : Uuid,
    name : String,
    email : String,
    created_at : chrono::DateTime<chrono::Utc>,
}

#[derive(Debug,Deserialize)]
struct CreateUser {
    name : String,
    email : String
}

#[derive(Debug, Deserialize)]
struct UpdateUser {
    name : Option<String>,
    email : Option<String>
}

// err handling
#[derive(Debug, thiserror::Error)]
enum DbErr {
    #[error("User Not Found")]
    NotFound,
    #[error("Database Error : {0}")]
    Sqlx(#[from] sqlx::Error)
}

impl IntoResponse for DbErr {
    fn into_response(self) -> axum::response::Response {
        let (_s, _msg) = match self {
            DbErr::NotFound => (StatusCode::NOT_FOUND, "User Not Found"),
            DbErr::Sqlx(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Db Error")
        };
        (_s, _msg).into_response()
    }
}

async fn ls_users(
    State(pool) : State<PgPool>
) -> Result<Json<Vec<User>>, DbErr> {
    let val = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await?;
    Ok(Json(val))
}

async fn get_users(
    State(pool) : State<PgPool>,
    Path(id) : Path<Uuid>
) -> Result<Json<User>, DbErr> {
    let val = sqlx::query_as::<_,User>("
    SELECT * FROM users WHERE id = $1"
    )
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(DbErr::NotFound)?;
    Ok(Json(val))
}

async fn create_user(
    State(pool) : State<PgPool>,
    Json(input) : Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), DbErr> {
    let val = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, name, email, created_at) VALUES ($1, $2, $3, NOW()) RETURNING *"
    )
        .bind(Uuid::new_v4())
        .bind(&input.name)
        .bind(&input.email)
        .fetch_one(&pool)
        .await?;
    Ok((StatusCode::CREATED, Json(val)))
}

async fn update_user(
    State(pool) : State<PgPool>,
    Path(id) : Path<Uuid>,
    Json(input) : Json<UpdateUser>
) -> Result<Json<User>, DbErr> {
    let val = sqlx::query_as::<_, User>(
        "UPDATE users SET name = COALESCE($2, name),
        email = COALESCE($3, email) WHERE id = $1 RETURNING *"
    )
        .bind(id)
        .bind(&input.name)
        .bind(&input.email)
        .fetch_optional(&pool)
        .await?
        .ok_or(DbErr::NotFound)?;
    Ok(Json(val))
}

async fn delete_user(
    State(pool) : State<PgPool>,
    Path(id) : Path<Uuid>,
) -> Result<StatusCode, DbErr> {
    let val = sqlx::query(
        "DELETE FROM users WHERE id = $1"
    )
        .bind(id)
        .execute(&pool)
        .await?;
    if val.rows_affected() == 0 {
        Err(DbErr::NotFound)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

//main
#[tokio::main]
async fn main(){
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DB_URL")
        .unwrap_or_else(|_|"postgres://postgres@localhost/axum_course".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("failed to connect db");

    // migrations
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()      
    )"
    )
    .execute(&pool)
    .await
    .expect("failed to create table");

    let app = Router::new()
    .route(
        "/users",
        get(ls_users)
        .post(create_user)
    )
    .route(
        "/users/{id}",
        get(get_users)
        .delete(delete_user)
        .put(update_user)
    )
    .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("[INFO] server running on http://localhost:3000/");

    axum::serve(listener, app).await.unwrap();
}






