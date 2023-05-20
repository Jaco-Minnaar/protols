mod backend;
mod logger;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use logger::create_logger;
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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    create_logger("protols.log", args.log_level.into()).expect("Failed to create logger");

    log::info!("Starting protols language server");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    log::info!("creating lsp service");

    let (service, socket) = LspService::new(|client| ProtoLanguageServer::new(client));

    log::info!("starting server");

    Server::new(stdin, stdout, socket).serve(service).await;

    log::info!("Shutting down protols language server");

    Ok(())
}
