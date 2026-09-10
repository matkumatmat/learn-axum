#[tokio::test]
async fn quick_dev() -> Result<(), Box<dyn std::error::Error>> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/string").await?.print().await?;
    hc.do_get("/owned").await?.print().await?;
    hc.do_get("/status").await?.print().await?;
    hc.do_get("/json/user").await?.print().await?;
    hc.do_get("/json/users").await?.print().await?;

    Ok(())
}
