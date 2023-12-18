mod backend;
mod logger;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use logger::create_logger;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpListener,
};
use tower_lsp::{LspService, Server};

use crate::backend::ProtoLanguageServer;

#[derive(Debug, ValueEnum, Clone, Copy)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Into<log::LevelFilter> for LogLevel {
    fn into(self) -> log::LevelFilter {
        match self {
            Self::Trace => log::LevelFilter::Trace,
            Self::Debug => log::LevelFilter::Debug,
            Self::Info => log::LevelFilter::Info,
            Self::Warn => log::LevelFilter::Warn,
            Self::Error => log::LevelFilter::Error,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The log level to use
    #[arg(value_enum, long, default_value = "info")]
    log_level: LogLevel,

    /// Use stdio for communication
    #[arg(long)]
    stdio: bool,
}

async fn start_server<I, O>(input: I, output: O) -> Result<()>
where
    I: AsyncRead + Unpin,
    O: AsyncWrite,
{
    log::info!("creating lsp service");

    let (service, socket) = LspService::new(|client| ProtoLanguageServer::new(client));

    log::info!("starting server");

    Server::new(input, output, socket).serve(service).await;

    log::info!("Shutting down protols language server");

    Ok(())
}

async fn start_stdio_server() -> Result<()> {
    log::info!("Starting protols in stdio mode");

    start_server(tokio::io::stdin(), tokio::io::stdout()).await
}

async fn start_tcp_server() -> Result<()> {
    log::info!("Starting protols in tcp mode");

    let listener = TcpListener::bind("127.0.0.1:50051").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        log::info!("Accepted connection from {:?}", socket.peer_addr());

        let (reader, writer) = tokio::io::split(socket);

        tokio::spawn(async move {
            if let Err(err) = start_server(reader, writer).await {
                log::error!("Error in tcp server: {}", err);
            }

            log::info!("Connection closed");
        });
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    create_logger("protols.log", args.log_level.into()).expect("Failed to create logger");

    log::info!("Starting protols language server");

    if args.stdio {
        start_stdio_server().await
    } else {
        start_tcp_server().await
    }
}
