use anyhow::{Error, Result};
use noticube::server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    server::run().await
}
