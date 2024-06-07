use noticube::server;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    let running_server = server::run().await;
    if let Err(e) = running_server {
        error!("{}", e);
        std::process::exit(1);
    }

    info!("shutdown was succeed");
    std::process::exit(0);
}
