use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name="orvpass",
    version="0.2.0",
    about="Secure local-first password manager"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Unlock,
    Add,
    Get,
    Generate,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => println!("Initialize vault"),
        Commands::Unlock => println!("Unlock vault"),
        Commands::Add => println!("Add credential"),
        Commands::Get => println!("Retrieve credential"),
        Commands::Generate => println!("Generate password"),
    }
}
