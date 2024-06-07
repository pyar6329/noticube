use super::ServerError;
use crate::client::SlackClient;
use anyhow::{anyhow, Error, Result};
use mailin_embedded::{response, Handler, Response, Server};
use std::sync::{Arc, Mutex};
use std::{io, net::IpAddr};
use tokio::{
    sync::{mpsc, oneshot},
    time::Duration,
};
use tracing::{debug, error, info};

#[derive(Debug, Clone)]
pub(super) struct SMTPServer {
    email_content_buffer: Arc<Mutex<String>>,
    tx: mpsc::Sender<String>,
}

impl Handler for SMTPServer {
    fn mail(&mut self, _ip: IpAddr, _domain: &str, from: &str) -> Response {
        debug!("received email. from: {}", from);
        response::OK
    }

    fn data(&mut self, buf: &[u8]) -> io::Result<()> {
        let maybe_line = std::str::from_utf8(buf);
        if let Ok(line) = maybe_line {
            let maybe_buffer = self.email_content_buffer.lock();
            if let Ok(mut buffer) = maybe_buffer {
                buffer.push_str(line);
            }
        }
        Ok(())
    }

    fn data_end(&mut self) -> Response {
        let maybe_buffer = self.email_content_buffer.lock();
        if let Ok(buffer) = maybe_buffer {
            debug!("result: {}", buffer);
            let tx2 = self.tx.to_owned();
            let buffer2 = buffer.to_owned();

            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();

            rt.spawn(async move { tx2.send(buffer2).await });
            rt.shutdown_timeout(Duration::from_secs(30));
        }
        response::OK
    }
}

impl SMTPServer {
    pub async fn run(address: &str, slack_client: &SlackClient) -> Result<(), Error> {
        info!("succeed loading configuration");
        let (ps_tx, ps_rx) = oneshot::channel();
        let (tx, mut rx) = mpsc::channel(32);
        let handler = SMTPServer {
            email_content_buffer: Arc::new(Mutex::new("".to_string())),
            tx,
        };
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
        tokio::spawn(async move {
            let server_status = server.serve();

            if let Err(e) = server_status {
                error!("error running SMTP server: {}", e);
                let _ = ps_tx.send(Err(ServerError::CannotStartServer));
            } else {
                let _ = ps_tx.send(Ok(()));
            }
        });

        let slack_client2 = slack_client.to_owned();
        tokio::spawn(async move {
            while let Some(content) = rx.recv().await {
                debug!("received content: {}", &content);
                let result = slack_client2.send(&content).await;
                if let Err(e) = result {
                    error!("failed sending content to Slack: {}", e);
                } else {
                    info!("succeed sending content to Slack");
                }
            }
        });

        if let Err(e) = ps_rx.await? {
            return Err(anyhow!(e));
        }

        Ok(())
    }
}
