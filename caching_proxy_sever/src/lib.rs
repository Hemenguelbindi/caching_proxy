use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CachingProxyServer {
    origin: String,
    cache: Arc<Mutex<HashMap<String, (Vec<u8>, String)>>>,
}


impl CachingProxyServer  {
    pub fn new(origin: String) -> Self {
        Self {
            origin,
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }


    pub async fn run(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        println!("Кэширующий прокси-сервер запущен на порту {}", port);

        loop {
            let (mut socket, _) = listener.accept().await?;
            let proxy = self.clone();

            tokio::spawn(async move {
                let mut buffer = [0u8; 1024];
                let n = socket.read(&mut buffer).await.expect("Ошибка чтения запроса");
                let request = String::from_utf8_lossy(&buffer[..n]);
                
                if let Some(response) = proxy.handle_request(request.to_string()).await {
                    socket.write_all(&response).await.expect("Ошибка записи ответа");
                }
            });
        }
    }

    async fn handle_request(&self, request: String) -> Option<Vec<u8>> {
        let path = parse_path(&request)?;
        let cache_key = format!("{}{}", self.origin, path);

        if let Some((cached_response, _)) = self.cache.lock().unwrap().get(&cache_key) {
            println!("Ответ из кэша: {}", cache_key);
            return Some(add_cache_header(cached_response.clone(), "HIT"));
        }

        println!("Ответ с сервера: {}", cache_key);
        let client = Client::new();
        match client.get(&cache_key).send().await {
            Ok(resp) => {
                let body = resp.bytes().await.expect("Ошибка при чтении тела ответа");
                let body_vec = body.to_vec();
                self.cache.lock().unwrap().insert(cache_key.clone(), (body_vec.clone(), "MISS".to_string()));
                Some(add_cache_header(body_vec, "MISS"))
            }
            Err(_) => None,
        }
    }

    pub fn clear_cache(&self) {
        self.cache.lock().unwrap().clear();
        println!("Cache clear");
    }
}

fn add_cache_header(mut response: Vec<u8>, cache_status: &str) -> Vec<u8> {
    response.extend(format!("\r\nX-Cache: {}", cache_status).as_bytes());
    response
}

fn parse_path(request: &str) -> Option<String> {
    request.lines().next()?.split_whitespace().nth(1).map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpStream;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use std::time::Duration;

    #[tokio::test]
    async fn test_cache_miss_and_hit() {
        let origin = "https://dummyjson.com".to_string();
        let proxy = CachingProxyServer::new(origin.clone());
        let port = 3000;

        tokio::spawn(async move {
            proxy.run(port).await.unwrap();
        });

        tokio::time::sleep(Duration::from_secs(1)).await;

        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
        let request = "GET /test HTTP/1.1\r\nHost: localhost\r\n\r\n";
        stream.write_all(request.as_bytes()).await.unwrap();

        let mut buffer = [0u8; 1024];
        let n = stream.read(&mut buffer).await.unwrap();
        let response = String::from_utf8_lossy(&buffer[..n]);
        assert!(response.contains("X-Cache: MISS"));

        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
        stream.write_all(request.as_bytes()).await.unwrap();

        let n = stream.read(&mut buffer).await.unwrap();
        let response = String::from_utf8_lossy(&buffer[..n]);
        assert!(response.contains("X-Cache: HIT"));
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let origin = "https://dummyjson.com".to_string();
        let proxy = CachingProxyServer::new(origin.clone());

        {
            let mut cache = proxy.cache.lock().unwrap();
            cache.insert(
                format!("{}/products", origin),
                (b"cached response".to_vec(), "HIT".to_string()),
            );
        }

        assert!(!proxy.cache.lock().unwrap().is_empty());
        proxy.clear_cache();
        assert!(proxy.cache.lock().unwrap().is_empty());
    }
}
