//! # Zenobuf CLI - Command-line tools for the Zenobuf framework
//!
//! The Zenobuf CLI provides essential tools for developing, debugging, and monitoring
//! Zenobuf applications. It allows you to inspect running systems, monitor message
//! flows, call services, and manage parameters.
//!
//! ## Installation
//!
//! ```bash
//! cargo install zenobuf-cli
//! ```
//!
//! ## Usage
//!
//! ### List System Components
//!
//! ```bash
//! # List all active nodes
//! zenobuf-cli list nodes
//!
//! # List all active topics
//! zenobuf-cli list topics
//!
//! # List all available services
//! zenobuf-cli list services
//! ```
//!
//! ### Monitor Topics
//!
//! ```bash
//! # Monitor messages on a topic
//! zenobuf-cli monitor sensor_data
//!
//! # Monitor with custom timeout
//! zenobuf-cli monitor sensor_data --timeout 30
//! ```
//!
//! ### Call Services
//!
//! ```bash
//! # Call a service with JSON data
//! zenobuf-cli call add_service --data '{"a": 5, "b": 3}'
//!
//! # Call with custom timeout
//! zenobuf-cli call status_service --timeout 10
//! ```
//!
//! ### Manage Parameters
//!
//! ```bash
//! # Get a parameter value
//! zenobuf-cli param get max_speed
//!
//! # Set a parameter value
//! zenobuf-cli param set max_speed 15.0
//!
//! # List all parameters
//! zenobuf-cli param list
//! ```
//!
//! ## Examples
//!
//! ### Development Workflow
//!
//! ```bash
//! # 1. Check what's running
//! zenobuf-cli list nodes
//! zenobuf-cli list topics
//!
//! # 2. Monitor your application's messages
//! zenobuf-cli monitor /robot/sensors/camera &
//! zenobuf-cli monitor /robot/control/velocity &
//!
//! # 3. Test services manually
//! zenobuf-cli call /robot/navigation/goto --data '{"x": 1.0, "y": 2.0}'
//!
//! # 4. Adjust parameters on the fly
//! zenobuf-cli param set /robot/max_speed 2.0
//! ```
//!
//! ### Debugging
//!
//! ```bash
//! # Check if your publisher is working
//! zenobuf-cli list topics | grep my_topic
//! zenobuf-cli monitor my_topic
//!
//! # Verify service availability
//! zenobuf-cli list services | grep my_service
//! zenobuf-cli call my_service --data '{}'
//!
//! # Check parameter values
//! zenobuf-cli param list | grep config
//! zenobuf-cli param get /app/config/debug_mode
//! ```

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
