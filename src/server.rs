mod smtp;

use crate::env::Config;
use anyhow::{Error, Result};
use smtp::*;
use thiserror::Error as ThisError;
use tracing::{error, info};
use tracing_subscriber;

#[derive(Debug, ThisError)]
enum ServerError {
    #[error("Invalid Configuration")]
    InvalidConfig,
    #[error("Cannot start server, Please check port number is not in use")]
    CannotStartServer,
}

pub fn run() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_current_span(false)
        .flatten_event(true)
        .with_span_list(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("loading configuration");
    let config = Config::new().map_err(|e| {
        error!("error loading configuration: {}", e);
        ServerError::InvalidConfig
    })?;

    SMTPServer::run(&config.get_address())?;
    Ok(())
}
