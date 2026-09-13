use serde_json::json;

#[tokio::test]
async fn test_08() -> Result<(), Box<dyn std::error::Error>> {
    let c = httpc_test::new_client("http://localhost:3000")?;
    c.do_get("/users").await?.print().await?;
    c.do_post(
        "/users",
        json!({
            "name" : "kayesssss",
            "email" : "112@gmail.com"
        }),
    )
    .await?
    .print()
    .await?;
    c.do_delete("/users/830b2f0a-e039-4c79-bc08-0972fd0f2bf5")
        .await?
        .print()
        .await?;

    println!("success");
    Ok(())
}

