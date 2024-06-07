use anyhow::{Error, Result};
use mailin_embedded::{response, Handler, Response, Server};
use std::net::IpAddr;
use thiserror::Error as ThisError;

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
        println!("yearrrrrrrrrrrrrrrrrr!!!!!!!: ");
        response::OK
    }
}

pub fn run() -> Result<(), Error> {
    let handler = MyHandler {};
    let mut server = Server::new(handler);

    server
        .with_name("example.com")
        .with_addr("0.0.0.0:50012")
        .map_err(|_| ServerError::InvalidConfig)?;
    let _ = server.serve().map_err(|_| ServerError::CannotStartServer)?;
    Ok(())
}
