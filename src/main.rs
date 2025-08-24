use std::{error::Error, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use libunftp::options::Shutdown;
use tokio::signal;
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
    .shutdown_indicator(shutdown())
    .greeting("Welcome to WebDAV FTP Server")
    .passive_ports(50000..=50005)
    .build()?;

    let addr = "0.0.0.0:2121";
    println!("Starting FTP server on {}", addr);
    println!("Press Ctrl+C to stop the server");

    server.listen(addr).await?;

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

async fn shutdown() -> Shutdown {
    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        }
    }
    libunftp::options::Shutdown::new().grace_period(Duration::from_secs(5))
}
