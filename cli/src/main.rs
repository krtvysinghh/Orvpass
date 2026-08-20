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
        Commands::Init => commands::init::run(),
        Commands::Unlock => commands::unlock::run(),
        Commands::Add => commands::add::run(),
        Commands::Get => commands::get::run(),
        Commands::Generate => commands::generate::run(),
    }
}

mod commands {
    pub mod init {
        pub fn run() {
            println!("Vault initialization coming next");
        }
    }

    pub mod unlock {
        pub fn run() {
            println!("Vault unlock coming next");
        }
    }

    pub mod add {
        pub fn run() {
            println!("Credential storage coming next");
        }
    }

    pub mod get {
        pub fn run() {
            println!("Credential retrieval coming next");
        }
    }

    pub mod generate {
        pub fn run() {
            println!("Secure password generation ready");
        }
    }
}
