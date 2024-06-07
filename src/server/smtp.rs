use super::ServerError;
// use crate::client::SlackClient;
use anyhow::{Error, Result};
use mailin_embedded::{response, Handler, Response, Server};
use std::{io, net::IpAddr};
use tracing::{error, info};

#[derive(Clone)]
pub(super) struct SMTPServer;

impl Handler for SMTPServer {
    fn mail(&mut self, _ip: IpAddr, _domain: &str, _from: &str) -> Response {
        info!("yearrrrrrrrrrrrrrrrrr!!!!!!!: ");
        response::OK
    }

    fn data(&mut self, buf: &[u8]) -> io::Result<()> {
        let maybe_str = std::str::from_utf8(buf);
        if let Ok(s) = maybe_str {
            info!("content: {}", s);
        } else {
            info!("errrrrrrrrrrrrrrrrr");
        }
        // client = SlackClient::new(&config.slack_bot_token, &config.slack_channel_id)?;
        Ok(())
    }
}

impl SMTPServer {
    pub fn run(address: &str) -> Result<(), Error> {
        info!("succeed loading configuration");
        let handler = SMTPServer {};
        let mut server = Server::new(handler);

        info!("checking SMTP server configuration");
        server
            .with_name("example.com")
            .with_addr(address)
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
}
