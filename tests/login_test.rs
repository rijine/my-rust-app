use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn login_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let login_req = hc.do_post("/api/login", json!({
        "username": "user",
        "password": "pass"
    }));

    login_req.await?.print().await?;
    Ok(())
}

#[tokio::test]
async fn login_test_failed() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let login_req = hc.do_post("/api/login", json!({
        "username": "user1",
        "password": "pass"
    }));

    login_req.await?.print().await?;
    Ok(())
}
