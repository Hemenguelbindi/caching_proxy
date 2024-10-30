use clap::Parser;
use caching_proxy_server::CachingProxyServer;
use tokio::runtime::Runtime;

#[derive(Debug, Parse)]
#[command(author="Hemenguelbindi", version="0.0.1", about="CLI Caching proxy server", long_about = None)]
pub struct Cli {
    #[arg(long)]
    port: String,
    #[arg(long)]
    origin: String,

    #[arg(long)]
    clear_cache: bool,
}


fn main() {

    let cli = Cli::parse();

    let caching_server = CachingProxyServer::new(cli.origin);

    let rt = Runtime::new().expect("Не удалось создать runtime");

    if cli.clear_cache {
        caching_server.clear_cache();
        println!("Кэш очищен.");
    } else {
        println!("Запуск кэширующего прокси-сервера на порту {}", cli.port);
        rt.block_on(caching_server.run(cli.port))
            .expect("Ошибка при запуске сервера");
    }
}