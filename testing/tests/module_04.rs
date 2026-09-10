use std::time::Duration;

#[tokio::test]
async fn quick_dev() -> Result<(), Box<dyn std::error::Error>> {
    tokio::time::sleep(Duration::from_secs(3)).await;
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/string").await?.print().await?;
    hc.do_get("/owned").await?.print().await?;
    // hc.do_get("/status").await?.print().await?;
    // hc.do_get("/json/user").await?.print().await?;
    // hc.do_get("/json/users").await?.print().await?;
    // hc.do_get("/json/created").await?.print().await?;
    // hc.do_get("/html").await?.print().await?;
    // hc.do_get("/html/dynamic").await?.print().await?;
    // hc.do_get("/headers").await?.print().await?;
    // hc.do_get("/full").await?.print().await?;
    // hc.do_get("/redirect/permanent").await?.print().await?;
    // hc.do_get("/redirect/temp").await?.print().await?;
    // hc.do_get("/custom").await?.print().await?;
    // hc.do_get("/api/success").await?.print().await?;
    // hc.do_get("/api/error").await?.print().await?;
    // hc.do_get("/maybe-error").await?.print().await?;
    hc.do_get("/ip").await?.print().await?;
    hc.do_get("/ip/json").await?.print().await?;
    
    Ok(())
}
