use serde::Deserialize;
use uuid::Uuid;
use axum::{Router, extract::Path, routing::{get, post}};
#[derive(Deserialize,Debug)]
pub struct UserRepository {
    user_id: Uuid,
    post_id:Option<Uuid>,
    comment_id:Option<Uuid>
}

pub async fn get_user(
    Path(id): Path<Uuid>
) -> String {
    format!("Getting User id with id : {}", id)
}
pub async fn get_user_posts(
    Path((user_id, post_id)): Path<(Uuid, Uuid)>
) -> String {
    format!("Getting User Post with user_id : {} and post id : {}",
    user_id, post_id)
}
pub async fn get_user_comment(
    Path(params) : Path<UserRepository> 
) -> String {
    format!(
        "User : {}
         Post : {:?}
         Comment: {:?}",
         params.user_id,
         params.post_id,
         params.comment_id
    )
}

pub async fn create_user() -> &'static str {
    "(POST) => Creating new user"
}
pub async fn update_user(
    Path(id): Path<Uuid>
) -> String {
    format!("(PUT) => updating new user [full update] {}",id)
}
pub async fn patch_user(
    Path(id): Path<Uuid>
) -> String {
    format!("(PATCH) => updating new user [partial update] {}",id)
}
pub async fn delete_user(
    Path(id) : Path<Uuid>
) -> String {
    format!("(DELETE) => deleting user {}",id)
}
pub async fn list_user() -> &'static str {
    "Listing All of Users"
}
pub async fn list_posts(
    Path(id) : Path<Uuid> 
) -> String {
    format!("getting Post : {}",id)
}
pub async fn get_posts(
    Path(id) : Path<Uuid> 
) -> String {
    format!("Getting Posts {}", id)
}

pub fn user_routes() -> Router{
    Router::new()
        .route("/", 
        get(list_user).post(create_user))
        .route("/{id}",
    get(get_user).put(update_user).patch(patch_user).delete(delete_user))
}
