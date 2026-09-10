// state management
// TODO :
//! - Immutable shared state with State<T>
//! - Mutable shared state with Arc<Mutex<T>>
//! - Database connection pools
//! - Multiple state types


use axum::{
    extract::State,
    http::StatusCode,
    routing::{get},
    Json,
    Router,
    Extension
};
use serde::{Deserialize,Serialize};
use std::{collections::HashMap,sync::{Arc,RwLock}};
use uuid::Uuid;


// create config
#[derive(Clone)]
struct ApplicationConfig{
    app_name : String,
    version : String,
    max_items_per_page : usize
}


// immutable share state
async fn get_cfg(
    State(config) : State<Arc<ApplicationConfig>>
) -> Json<serde_json::Value> {
    Json(
        serde_json::json!({
            "app_name" : config.app_name,
            "version" : config.version,
            "max_items_per_page" : config.max_items_per_page
        })
    )
}

// mutable share state
// simple in-memory database of Todos
// menggunakan RwLock untuk performa (multiple reader, single writer)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todos{
    id: String,
    title : String,
    status: bool
}

#[derive(Debug, Deserialize)]
struct CreateTodos {
    title : String
}

#[derive(Debug, Deserialize)]
struct UpdateTodos {
    title : Option<String>,
    status : Option<bool>
}

// mutable state ; thread safe hashmap
type Todostore = Arc<RwLock<HashMap<String, Todos>>>;

// list semua todo
async fn ls_todos (
    State(store) : State<Todostore>
) -> Json<Vec<Todos>>{
    let todos = store.read().unwrap();
    let todos_vec : Vec<Todos> = todos
        .values()
        .cloned()
        .collect();
    Json(todos_vec)
}

// create todos
async fn create_todos(
    State(store) : State<Todostore>,
    Json(input) : Json<CreateTodos>
) -> (StatusCode, Json<Todos>) {
    let value = Todos {
        id : Uuid::now_v7().to_string(),
        title : input.title,
        status: false
    };
    store
        .write()
        .unwrap()
        .insert(value.id.clone(), value.clone());
    (StatusCode::CREATED, Json(value))
}

// single get todos with {id}
async fn get_todos(
    State(store) : State<Todostore>,
    axum::extract::Path(id) : axum::extract::Path<String>,
) -> Result<Json<Todos>, StatusCode> {
    let value = store
        .read()
        .unwrap();
    value
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// update todos
async fn update_todos(
    State(store) : State<Todostore>,
    axum::extract::Path(id) : axum::extract::Path<String>,
    Json(input) : Json<UpdateTodos>
) -> Result<Json<Todos>, StatusCode> {
    let mut value = store
        .write()
        .unwrap();

    if let Some(values) = value.get_mut(&id) {
        if let Some(title) = input.title {
            values.title = title;
        }
        if let Some(status) = input.status {
            values.status = status;
        }
        Ok(Json(values.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// delete todos
async fn delete_todos (
    State(store) : State<Todostore>,
    axum::extract::Path(id) : axum::extract::Path<String>,
) -> StatusCode {
    let mut values = store
        .write()
        .unwrap();
    if values.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}



// multiple state types
#[derive(Clone)]
#[allow(dead_code)]
struct CombinedState{
    cfg : Arc<ApplicationConfig>,
    todos : Todostore,
    metrics : Arc<RwLock<Metrics>>
}

#[derive(Debug,Default)]
struct Metrics {
    req_count : u64,
    err_count : u64
}

// You can extract the whole state or use From traits for convenience
async fn get_metric(
    State(state) : State<CombinedState>,
) -> Json<serde_json::Value> {
    let values = state
        .metrics
        .read()
        .unwrap();
    Json(serde_json::json!({
        "req" : values.req_count,
        "err" : values.err_count,
        "app_v" : state.cfg.version
    }))
}

async fn increment_request_count(
    State(state) : State<CombinedState>
) -> &'static str {
    let mut values = state
        .metrics
        .write()
        .unwrap();
    values.req_count += 1;
    "request counted"
}


// database conn pool pattern
// kalo di asli struct nya pake sqlx::PgPool dan sejenisnya
#[derive(Clone)]
#[allow(dead_code)]
struct DbPool {
    conn_string : String,
    max_conn : u32
}
impl DbPool {
    fn new(conn_string : &str) -> Self {
        Self {
            conn_string: conn_string.to_string(),
            max_conn : 10
        }
    }
    async fn query(
        &self,
        _sql:&str
    ) -> Result<Vec<String>, String> {
        // In real app: sqlx::query!(...).fetch_all(&self.pool).await
        Ok(vec!["Result1".to_string(), "Result2".to_string()])
    }
}

async fn db_query(
    State(pool): State<DbPool>
) -> Json<Vec<String>> {
    match pool.query("SELECT * FROM users").await {
        Ok(results) => Json(results),
        Err(_) => Json(vec![])
    }
}

// state extensions patterns
#[derive(Clone)]
struct CurrentUser {
    id: String,
    name: String,
}

async fn get_current_user(Extension(user): Extension<CurrentUser>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "id": user.id,
        "name": user.name
    }))
}

#[tokio::main]
async fn main() {
    let cfg = Arc::new(ApplicationConfig{
        app_name: "serve one".to_string(),
        version : "0.0.0.0.1".to_string(),
        max_items_per_page : 100
    });
    // Initialize mutable todo store
    let todo_store : Todostore = Arc::new(RwLock::new(HashMap::new()));

    {
        let mut store = todo_store
            .write()
            .unwrap();
        let todo = Todos{
            id: Uuid::now_v7().to_string(),
            title: "Learning axum".to_string(),
            status: false
        };
        store.insert(todo.id.clone(), todo);
    }

    let combined_state = CombinedState {
        cfg: cfg.clone(),
        todos: todo_store.clone(),
        metrics: Arc::new(RwLock::new(Metrics::default()))
    };

    let db_pool = DbPool::new("postgres://localhost/prikitiw");

        // Current user (normally set by auth middleware)
    let current_user = CurrentUser {
        id: "user-123".to_string(),
        name: "Demo User".to_string(),
    };

    // Build routes for todo CRUD
    let todos_routes: Router<()> = Router::new()
        .route("/", get(ls_todos).post(create_todos))
        .route("/{id}", get(get_todos).put(update_todos).delete(delete_todos))
        .with_state(todo_store);

    let cfg_routes: Router<()> = Router::new()
        .route("/config", get(get_cfg))
        .with_state(cfg);

    let combined_routes: Router<()> = Router::new()
        .route("/metrics", get(get_metric))
        .route("/track", get(increment_request_count))
        .with_state(combined_state);

    let db_routes: Router<()> = Router::new()
        .route("/db/users", get(db_query))
        .with_state(db_pool);

    let app = Router::new()
        .merge(todos_routes) 
        .merge(cfg_routes)
        .merge(combined_routes)
        .merge(db_routes)
        .route("/me", get(get_current_user))
        .layer(Extension(current_user));





    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}