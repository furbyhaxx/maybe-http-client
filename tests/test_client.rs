use http_client::{HttpClient, Query};

#[maybe_async::test(
    feature="sync",
    async(all(not(feature="sync"), feature="async"), async_std::test),
)]
async fn test_http_client() {
    let client = HttpClient::default();
    let query = Query::new();
    let result = client.get("https://dummy.restapiexample.com/api/v1/employees", None, &query).await;
    println!("");
}