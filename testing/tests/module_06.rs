use httpc_test::new_client_with_reqwest;
use reqwest::header::{HeaderMap, HeaderValue};

#[tokio::test]
async fn test_06() -> Result<(), Box<dyn std::error::Error>> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/protected/data").await?.print().await?;

    let mut headers = HeaderMap::new();
    headers.insert("X-API-KEY", HeaderValue::from_static("secret-key"));
    let auth_builder = reqwest::Client::builder().default_headers(headers);
    let hc_auth = new_client_with_reqwest("http://localhost:3000", auth_builder)?;
    hc_auth.do_get("/protected/data").await?.print().await?;

    hc.do_get("/public").await?.print().await?;
    hc.do_get("/slow").await?.print().await?;

    Ok(())
}