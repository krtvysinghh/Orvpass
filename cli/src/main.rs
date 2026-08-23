mod clipboard;
mod password;
mod vault;
use clap::{Parser, Subcommand};

mod commands;
mod completion;
mod config;
mod output;

#[derive(Parser)]
#[command(
    name = "orvpass",
    version = "3.0.0",
    about = "Secure offline password manager"
)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]

enum Commands {
    List,

    Get {
        name: String,
    },

    Remove {
        name: String,
    },

    Status,

    Add {
        name: String,
    },

    Search {
        query: String,
    },

    Generate {
        #[arg(short, long, default_value_t = 20)]
        length: usize,
    },

    Import {
        file: String,
    },

    Export {
        file: String,
    },

    Version,

    Init,
    Unlock,
}

fn main() {
    let cfg = config::CliConfig::load();
    output::header("Orvpass");
    output::info(&format!("Theme: {}", cfg.theme));
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            commands::init::execute();
        }

        Commands::Unlock => {
            commands::unlock::execute();
        }

        Commands::Add { name } => {
            commands::add::execute(name);
        }

        Commands::List => {
            commands::list::execute();
        }

        Commands::Get { name } => {
            commands::get::execute(&name);
        }

        Commands::Remove { name } => {
            commands::remove::execute(name);
        }

        Commands::Status => {
            commands::status::execute();
        }

        Commands::Search { query } => {
            commands::search::execute(&query);
        }

        Commands::Generate { length } => {
            let pass = commands::generate::execute(length);

            output::success(&pass);
        }

        Commands::Import { file } => {
            commands::import::execute(&file);
        }

        Commands::Export { file } => {
            commands::export::execute(&file);
        }

        Commands::Version => {
            println!("Orvpass 3.0.0");
        }
    }
}
