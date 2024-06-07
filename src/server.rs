use crate::env::Config;
use anyhow::{Error, Result};
use mailin_embedded::{response, Handler, Response, Server};
use std::net::IpAddr;
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

#[derive(Clone)]
struct MyHandler {}
impl Handler for MyHandler {
    fn mail(&mut self, _ip: IpAddr, _domain: &str, _from: &str) -> Response {
        info!("yearrrrrrrrrrrrrrrrrr!!!!!!!: ");
        response::OK
    }
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
    info!("succeed loading configuration");
    let handler = MyHandler {};
    let mut server = Server::new(handler);

    info!("checking SMTP server configuration");
    server
        .with_name("example.com")
        .with_addr(config.get_address())
        .map_err(|e| {
            error!("error configuring SMTP server: {}", e);
            ServerError::InvalidConfig
        })?;
    info!("succeed SMTP server configuration");
    info!("running SMTP server");
    let _ = server.serve().map_err(|e| {
        error!("error running SMTP server: {}", e);
        ServerError::CannotStartServer
    })?;
    Ok(())
}
