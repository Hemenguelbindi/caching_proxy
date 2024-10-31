use once_cell::sync::Lazy;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use reqwest::Client;
use std::collections::HashMap;
use tokio::sync::Mutex;

static CACHE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));


#[derive(Debug, Clone)]
pub struct CachingProxyServer {
    origin: String,
}

impl CachingProxyServer {
    pub fn new(origin: String) -> Self {
        Self { origin }
    }
    pub async fn run(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        println!("Кэширующий прокси-сервер запущен на порту {}", port);

        loop {
            let (mut socket, _) = listener.accept().await?;
            let server = self.clone();
            tokio::spawn(async move {
                let mut buffer = [0u8; 1024];
                let n = socket.read(&mut buffer).await.expect("Ошибка чтения запроса");
                let request = String::from_utf8_lossy(&buffer[..n]);

                if let Some(response) = server.handle_request(request.to_string()).await {
                    socket.write_all(&response).await.expect("Ошибка записи ответа");
                }
            });
        }
    }

    async fn handle_request(&self, request: String) -> Option<Vec<u8>> {
        let path = parse_path(&request)?;
        let cache_key = format!("{}{}", self.origin, path);
        
        if path == "/clear-cache" {
            self.clear_cache().await;
            return Some(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n".to_vec());
        }
        
        let cached_response = {
            let cache = CACHE.lock().await;
            cache.get(&cache_key).cloned()
        };
    
        if let Some(response) = cached_response {
            println!("HIT {}", cache_key);
            return Some(format_http_response(response.into_bytes(), "HIT"));
        }
    
        println!("MISS: {}", cache_key);
        let client = Client::new();
        match client.get(&cache_key).send().await {
            Ok(resp) => {
                let body = resp.text().await.expect("Ошибка при чтении тела ответа");
                let mut cache = CACHE.lock().await;
                cache.insert(cache_key.clone(), body.clone());
                Some(format_http_response(body.into_bytes(), "MISS"))
            }
            Err(_) => None,
        }
    }
    
    
    
    pub async fn clear_cache(&self) {
        let mut cache = CACHE.lock().await;
        cache.clear();
        println!("Кэш очищен.");
    }
}

fn format_http_response(body: Vec<u8>, cache_status: &str) -> Vec<u8> {
    let mut response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nX-Cache: {}\r\n\r\n",
        body.len(),
        cache_status
    )
    .into_bytes();
    response.extend(body);
    response
}

fn parse_path(request: &str) -> Option<String> {
    request.lines().next()?.split_whitespace().nth(1).map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::format_http_response;

    #[test]
    fn test_parse_path() {
        assert_eq!(parse_path("GET / HTTP/1.1"), Some("/".to_string()));
    }

    #[test]
    fn test_format_http_response() {
        let body = b"Hello, World!".to_vec();
        let cache_status = "HIT";
        let response = format_http_response(body.clone(), cache_status);

        // Преобразуем ответ обратно в строку для удобной проверки
        let response_str = String::from_utf8_lossy(&response);

        // Проверяем наличие правильных заголовков
        assert!(response_str.contains("HTTP/1.1 200 OK"));
        assert!(response_str.contains(&format!("Content-Length: {}", body.len())));
        assert!(response_str.contains(&format!("X-Cache: {}", cache_status)));

        // Проверяем, что тело ответа присутствует и правильно расположено после заголовков
        let expected_response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nX-Cache: {}\r\n\r\n{}",
            body.len(),
            cache_status,
            String::from_utf8_lossy(&body)
        );
        assert_eq!(response_str, expected_response);
    }
}