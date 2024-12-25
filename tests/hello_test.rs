use anyhow::Result;

#[tokio::test]
async fn hello() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello?name=Rocky").await?.print().await?;
    Ok(())
}