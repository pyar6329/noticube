use anyhow::{Error, Result};
use noticube::server;

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    server::run()
}
