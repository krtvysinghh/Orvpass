use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(about = "Orvpass Password Manager", version = "1.0.0-rc2")]
#[command(name = "orvpass")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add,
    List,
    Unlock,
    Status,
    Get,
    Generate,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run(),
        Commands::Add => commands::add::run(),
        Commands::List => commands::list::run(),
        Commands::Unlock => commands::unlock::run(),
        Commands::Status => commands::status::run(),

        Commands::Get => {
            use std::io::{self, Write};

            print!("Item ID: ");
            io::stdout().flush().unwrap();

            let mut id = String::new();
            io::stdin().read_line(&mut id).unwrap();

            commands::get::run(id.trim().to_string())
        }

        Commands::Generate => commands::generate::run(),
    }
}
