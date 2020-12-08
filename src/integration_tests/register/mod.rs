use super::setup::*;
use serde_json::json;

#[tokio::test(threaded_scheduler)]
async fn test_register_smoke_test() {
    let client = Client::new();

    let res = client.post("/register", json!({})).await;
    let got = res.cookie.is_some();
    let want = true;

    assert_eq!(got, want);
}
