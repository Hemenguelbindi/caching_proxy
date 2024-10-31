// tests/caching_proxy_server_test.rs
use caching_proxy_sever::CachingProxyServer;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_server_caching_behavior() -> Result<(), Box<dyn std::error::Error>> {
    let server = CachingProxyServer::new(String::from("http://dummyjson.com"));
    let port = 3000;
    
    let server_clone = server.clone();

    
    tokio::spawn(async move {
        server_clone.run(port).await.unwrap();
    });


    sleep(Duration::from_millis(100)).await;


    let client = Client::new();

    let response = client.get(&format!("http://127.0.0.1:{}/test", port))
        .send().await?;
    assert!(response.headers().get("X-Cache").unwrap().to_str()? == "MISS");

    let response = client.get(&format!("http://127.0.0.1:{}/test", port))
        .send().await?;
    assert!(response.headers().get("X-Cache").unwrap().to_str()? == "HIT");

    server.clear_cache().await;

    let response = client.get(&format!("http://127.0.0.1:{}/test", port))
        .send().await?;
    assert!(response.headers().get("X-Cache").unwrap().to_str()? == "MISS");

    Ok(())
}
