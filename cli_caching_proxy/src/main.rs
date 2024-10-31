use clap::Parser;
use caching_proxy_sever::CachingProxyServer;
use reqwest::Client;
use std::process;

#[derive(Debug, Parser)]
#[command(author="Hemenguelbindi", version="0.0.1", about="CLI Caching proxy server", long_about = None)]
pub struct Cli {
    #[clap(short,long, default_value = "3000")]
    port: u16,
    
    #[clap(short, long, default_value = "http://localhost")]
    origin: String,

    #[clap(long)]
    clear_cache: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.clear_cache {
        if let Err(e) = clear_cache(&cli.port).await {
            eprintln!("Ошибка при очистке кэша: {:?}", e);
            process::exit(1);
        }
        println!("Кэш успешно очищен.");
    } else {
        let caching_server = CachingProxyServer::new(cli.origin);
        caching_server.run(cli.port).await.unwrap();
        println!("Запуск прокси-сервера кэша на порту {}", cli.port);
        if let Err(e) = caching_server.run(cli.port).await {
            eprintln!("Ошибка при запуске сервера: {:?}", e);
            process::exit(1);
        }
    }
}

async fn clear_cache(port: &u16) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("http://127.0.0.1:{}/clear-cache", port);
    client.post(&url).send().await?;
    Ok(())
}
