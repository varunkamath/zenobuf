//! Command-line tools for the Zenobuf framework

use clap::{Parser, Subcommand};

mod commands;
mod error;

use error::Result;

/// Command-line tools for the Zenobuf framework
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Subcommand to run
    #[clap(subcommand)]
    command: Commands,
}

/// Subcommands for the Zenobuf CLI
#[derive(Subcommand)]
enum Commands {
    /// List nodes, topics, or services
    #[clap(subcommand)]
    List(commands::list::ListCommands),

    /// Monitor a topic
    Monitor(commands::monitor::MonitorArgs),

    /// Call a service
    Call(commands::call::CallArgs),

    /// Get or set a parameter
    #[clap(subcommand)]
    Param(commands::param::ParamCommands),
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse command-line arguments
    let cli = Cli::parse();

    // Execute the command
    match cli.command {
        Commands::List(cmd) => commands::list::execute(cmd).await?,
        Commands::Monitor(args) => commands::monitor::execute(args).await?,
        Commands::Call(args) => commands::call::execute(args).await?,
        Commands::Param(cmd) => commands::param::execute(cmd).await?,
    }

    Ok(())
}
