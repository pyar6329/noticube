use anyhow::{Error, Result};
use noticube::server;

fn main() -> Result<(), Error> {
    server::run()
}
