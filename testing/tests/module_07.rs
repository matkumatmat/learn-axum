#[tokio::test]
async fn test_07() -> Result<(), Box<dyn std::error::Error>>{
    let c = httpc_test::new_client("http://localhost:3000")?;
    // c.do_get("/users/0").await?.print().await?;
    // c.do_get("/users/1").await?.print().await?;
    // c.do_get("/users/2").await?.print().await?;
    // c.do_get("/users/3").await?.print().await?;
    // c.do_get("/users/101").await?.print().await?;
    // c.do_get("/validate/na").await?.print().await?;
    // c.do_get("/protected").await?.print().await?;
    c.do_get("/database").await?.print().await?;
    c.do_get("/complex/<script>alert(1)</script>").await?.print().await?;

    Ok(())
}