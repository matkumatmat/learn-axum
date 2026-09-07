use serde::Deserialize;
use axum::{Router,routing::{get},extract::Query};

use crate::user_service::{get_posts, list_posts};

#[derive(Debug,Deserialize)]
pub struct Pagination{
    page:Option<u32>,
    limit:Option<u32>
}

#[derive(Debug,Deserialize)]
pub struct SearchParams{
    q:String,
    category: Option<String>,
    sort: Option<String>
}

// pub async fn files(
//     Path(path) : Path<String>
// ) -> String {
//     format!("Accessing Files : {}", path)
// }

pub async fn list_items(
    Query(pagination) : Query<Pagination>
) -> String {
    let page = pagination.page.unwrap_or(1);
    let limit  = pagination.limit.unwrap_or(100);
    format!("Listing Items on Page : {} with limit : {} ", page,limit)
}
pub async fn search(
    Query(params): Query<SearchParams>
) -> String {
    format!(
        "Searching for '{}' in category '{}', sorted by '{}'",
        params.q,
        params.category.unwrap_or_else(|| "all".to_string()),
        params.sort.unwrap_or_else(|| "relevance".to_string())        
    )
}

pub async fn not_found() -> (axum::http::StatusCode, &'static str) {
    (axum::http::StatusCode::NOT_FOUND, "404 - Route not found")
}

pub fn post_routes() -> Router {
    Router::new()
        .route("/", get(list_posts))
        .route("/{id}", get(get_posts))
}
