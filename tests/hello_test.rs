use anyhow::Result;

#[tokio::test]
async fn hello() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello?name=Rocky").await?.print().await?;
    Ok(())
}

#[tokio::test]
async fn hello_path() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello_path/Dave").await?.print().await?;
    Ok(())
}

#[tokio::test]
async fn home_html() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/").await?.print().await?;
    Ok(())
}