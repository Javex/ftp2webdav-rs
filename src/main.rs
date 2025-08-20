use std::{error::Error, sync::Arc};

use anyhow::{Context, Result};
use unftp_sbe_webdav::{WebDavAuthenticator, WebDavBackend, WebDavUser};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logger();
    // Start the FTP server
    serve_ftp().await?;

    Ok(())
}

/// Start the FTP server with the given WebDAV backend
async fn serve_ftp() -> Result<(), Box<dyn Error>> {
    let server = libunftp::ServerBuilder::<WebDavBackend, WebDavUser>::with_authenticator(
        Box::new(move || WebDavBackend {}),
        Arc::new(webdav_authenticator()?),
    )
    .greeting("Welcome to WebDAV FTP Server")
    .passive_ports(50000..=65535)
    .build()?;

    println!("Starting FTP server on 127.0.0.1:2121");
    println!("Press Ctrl+C to stop the server");

    server.listen("127.0.0.1:2121").await?;

    Ok(())
}

/// Create and configure the WebDAV authenticator
fn webdav_authenticator() -> Result<WebDavAuthenticator> {
    let server = std::env::var("WEBDAV_SERVER")
        .context("failed to read environment variable `WEBDAV_SERVER`")?;

    Ok(WebDavAuthenticator { server })
}

fn init_logger() {
    let mut builder = env_logger::Builder::new();
    builder.filter_level(log::LevelFilter::Warn);
    builder.filter_module("unftp_sbe_webdav", log::LevelFilter::Debug);
    builder.init();
}
