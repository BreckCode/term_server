mod terminal;
mod utils;

use clap::Parser;
use colored::Colorize;
use std::net::Ipv4Addr;
use terminal::start_server;

const DEFAULT_PORT: u16 = 3000;
const LOCAL_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

#[derive(Parser)]
#[command(name = "server", version, author = "CodeKings Team", about = "CodeKings Terminal Server - A terminal server for code editors", long_about = None)]
struct Cli {
    /// Port to start the server
    #[arg(short, long, default_value_t = DEFAULT_PORT, value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,
}

fn print_startup_message() {
    println!("\n{}", "═".repeat(60).cyan());
    println!("{}", "  🚀 CodeKings Terminal Server Started!".bright_cyan().bold());
    println!("{}", "     Credit: Based on code reference from Bajarang Coder".bright_black());
    println!("{}", "     You can also use Bajarang Coder's AXS server on port 3000".bright_black());
    println!("{}\n", "═".repeat(60).cyan());
}

#[tokio::main]
async fn main() {
    let cli: Cli = Cli::parse();

    print_startup_message();

    let ip = LOCAL_IP;

    start_server(ip, cli.port).await;
}
