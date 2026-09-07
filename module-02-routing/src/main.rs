// create user post and comments posts rest API

use axum::{
    Router, routing::get
};

use crate::app_service::search;

mod user_service;
mod app_service;

pub fn v1() -> Router {
    Router::new()
        .nest("/users", user_service::user_routes())
        .nest("/posts", app_service::post_routes())
}

pub fn v2() -> Router {
    Router::new()
        .route("/users", get(|| async { "API v2 - Users endpoint" }))
        .route("/posts", get(|| async { "API v2 - Posts endpoint" }))
}

#[tokio::main]
async fn main(){
    let app= Router::new()
        .route("/",get(app_service::list_items))
        .route("/search", get(search))
        .nest("/api/v1", v1())
        .nest("/api/v2", v2())
        .fallback(app_service::not_found);
    let listener= tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("INTERNAL_SERVER_ERROR");
    axum::serve(listener, app).await.expect("internal server down");
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // buat .oneshot

    fn app() -> Router {
        Router::new()
            .route("/", get(app_service::list_items))
            .route("/search", get(search))
            .nest("/api/v1", v1())
            .nest("/api/v2", v2())
            .fallback(app_service::not_found)
    }

    async fn get_body(res: axum::response::Response) -> String {
        let bytes = axum::body::to_bytes(res.into_body(), 1024 * 1024).await.unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[tokio::test]
    async fn test_root_query() {
        let app = app();
        let req = Request::builder().uri("/?page=2&limit=5").body(Body::empty()).unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(get_body(res).await, "Listing Items on Page : 2 with limit : 5 ");
    }

    #[tokio::test]
    async fn test_search_ok() {
        let app = app();
        let req = Request::builder().uri("/search?q=axum&category=books").body(Body::empty()).unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert!(get_body(res).await.contains("axum"));
    }

    #[tokio::test]
    async fn test_search_missing_q() {
        let app = app();
        let req = Request::builder().uri("/search").body(Body::empty()).unwrap();
        let res = app.oneshot(req).await.unwrap();
        // harusnya 422 karena `q` required di SearchParams
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_v1_users_crud() {
        let app = app();
        let id = "550e8400-e29b-41d4-a716-446655440000";

        // GET /api/v1/users (tanpa slash!)
        let req = Request::builder().uri("/api/v1/users").body(Body::empty()).unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(get_body(res).await, "Listing All of Users");

        // GET /api/v1/users/{id}
        let req = Request::builder().uri(format!("/api/v1/users/{id}")).body(Body::empty()).unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert!(get_body(res).await.contains(id));

        // PUT / PATCH / DELETE
        for method in ["PUT", "PATCH", "DELETE"] {
            let req = Request::builder().method(method).uri(format!("/api/v1/users/{id}")).body(Body::empty()).unwrap();
            let res = app.clone().oneshot(req).await.unwrap();
            assert_eq!(res.status(), StatusCode::OK);
        }

        // POST
        let req = Request::builder().method("POST").uri("/api/v1/users").body(Body::empty()).unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(get_body(res).await, "(POST) => Creating new user");
    }

    #[tokio::test]
    async fn test_v1_users_invalid_uuid() {
        let app = app();
        let req = Request::builder().uri("/api/v1/users/123").body(Body::empty()).unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST); // 422
    }

    #[tokio::test]
    async fn test_v1_users_trailing_slash_404() {
        let app = app();
        let req = Request::builder().uri("/api/v1/users/").body(Body::empty()).unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND); // ini yang lo kena tadi
    }

    #[tokio::test]
    async fn test_v2() {
        let app = app();
        let req = Request::builder().uri("/api/v2/users").body(Body::empty()).unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(get_body(res).await, "API v2 - Users endpoint");

        let req = Request::builder().uri("/api/v2/posts").body(Body::empty()).unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(get_body(res).await, "API v2 - Posts endpoint");
    }

    #[tokio::test]
    async fn test_fallback() {
        let app = app();
        let req = Request::builder().uri("/ngarang").body(Body::empty()).unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}

