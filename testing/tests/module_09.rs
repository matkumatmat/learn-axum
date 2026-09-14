use axum::http::{HeaderMap, HeaderValue};
use httpc_test::new_client_with_reqwest;
use serde_json::json;


#[tokio::test]
async fn module_09() -> Result<(), Box<dyn std::error::Error>>{
    let c = httpc_test::new_client("http://localhost:3000")?;
    // c.do_get("/protected/me").await?.print().await?;
    // c.do_post("/register", json!({
    //     "name" : "a",
    //     "email" : "email",
    //     "pwd" : "pwn"
    // })).await?.print().await?;
    c.do_post("/login", json!({
        "email":"test@example.com",
        "pwd" : "password123"        
    })).await?.print().await?;
    let mut h = HeaderMap::new();
    h.insert("Authorization", HeaderValue::from_static("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyLTEiLCJleHAiOjE3ODk0NDQxMzQsInJvbGUiOiJ1c2VyIn0.cAMqi8Bll0e_M4FN6WzJzbnup5LKWLKEgl8S9IiUzio"));
    let auth_builder = reqwest::Client::builder().default_headers(h);
    let hc = new_client_with_reqwest("http://localhost:3000", auth_builder)?;
    hc.do_get("/protected/me").await?.print().await?;
    
    Ok(())
}


