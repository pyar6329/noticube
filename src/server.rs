mod smtp;

use crate::client::SlackClient;
use crate::env::Config;
use anyhow::{anyhow, Error, Result};
use smtp::*;
use thiserror::Error as ThisError;
use tracing::{error, info};
use tracing_subscriber;
use tracing_subscriber::EnvFilter;

#[derive(Debug, ThisError)]
enum ServerError {
    #[error("Invalid Configuration")]
    InvalidConfig,
    #[error("Cannot start server, Please check port number is not in use")]
    CannotStartServer,
    #[error("Something went wrong")]
    UnknownError,
}

pub async fn run() -> Result<(), Error> {
    let config = Config::new().map_err(|_| ServerError::InvalidConfig)?;

    let log_filter = if config.debug_mode {
        EnvFilter::from_default_env() // We can use: error!(), warn!(), info!(), debug!()
            .add_directive("noticube=debug".parse()?)
    } else {
        EnvFilter::from_default_env() // We can use: error!(), warn!(), info!()
            .add_directive("noticube=info".parse()?)
    };

    tracing_subscriber::fmt()
        .json()
        .with_current_span(false)
        .flatten_event(true)
        .with_span_list(true)
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(log_filter)
        .init();

    let slack_client = SlackClient::new(&config.slack_bot_token, &config.slack_channel_id)?;

    info!("starting noticube server");

    let server_address = config.get_address();
    tokio::select! {
        ctrl_c_result = tokio::signal::ctrl_c() => {
            if ctrl_c_result.is_ok() {
                info!("ctrl-c was inputted. shutdown server...");
                Ok(())
            } else {
                info!("ctrl-c was inputted, but something error occurred.");
                Err(anyhow!(ServerError::UnknownError))
            }

        }
        server_status = SMTPServer::run(&server_address, &slack_client) => {
            if let Err(e) = server_status {
                Err(e)
            } else {
                Ok(())
            }
        }
    }
}
