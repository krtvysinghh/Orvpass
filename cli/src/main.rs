use clap::{Parser, Subcommand};
use clap_complete::Shell;

mod clipboard;
mod commands;
mod completion;
mod config;
mod output;
mod password;
mod tui;
pub mod vault;

#[derive(Parser)]
#[command(
    name = "orvpass",
    version = "5.0.0",
    about = "⚡ Ultra-fast, zero-knowledge terminal password & secrets manager"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Launch interactive Ratatui terminal dashboard
    Tui,

    /// List all vault credentials formatted as table or JSON
    List {
        #[arg(short, long)]
        json: bool,
        #[arg(short, long)]
        category: Option<String>,
    },

    /// Retrieve credentials for an item (password, username, or TOTP)
    Get {
        name: String,
        #[arg(short, long)]
        password: bool,
        #[arg(short, long)]
        username: bool,
        #[arg(short, long)]
        totp: bool,
        #[arg(short, long)]
        copy: bool,
        #[arg(short, long)]
        json: bool,
    },

    /// Add a new credential or secret to the vault
    Add {
        title: Option<String>,
        #[arg(short, long)]
        username: Option<String>,
        #[arg(short, long)]
        password: Option<String>,
        #[arg(short, long)]
        note: Option<String>,
        #[arg(short, long)]
        category: Option<String>,
    },

    /// Remove a credential from the vault
    Remove {
        name: String,
    },

    /// Delete a credential from the vault
    Delete {
        name: String,
    },

    /// Fuzzy search credentials across all fields
    Search {
        query: String,
    },

    /// Calculate live RFC 6238 TOTP 2FA code
    Totp {
        name: String,
        #[arg(short, long)]
        watch: bool,
        #[arg(short, long)]
        copy: bool,
    },

    /// Generate cryptographically secure password or Diceware passphrase
    Generate {
        #[arg(short, long, default_value_t = 20)]
        length: usize,
        #[arg(short, long)]
        diceware: bool,
    },

    /// Run a command injecting vault secrets directly into child process RAM environment
    Run {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
    },

    /// 24/7 Watchdog security audit (weak, reused, breached credentials)
    Audit {
        #[arg(short, long)]
        json: bool,
    },

    /// Shamir's Secret Sharing (3-of-5 split or recovery)
    Sss {
        #[arg(short, long)]
        split: bool,
        #[arg(short, long)]
        recover: Option<Vec<String>>,
    },

    /// Create ephemeral, self-destructing end-to-end encrypted drop link
    Orvsend {
        text: String,
        #[arg(short, long, default_value_t = 24)]
        expires: u32,
        #[arg(short, long)]
        passphrase: Option<String>,
    },

    /// SSH key management and agent socket bridging
    Ssh {
        action: Option<String>,
    },

    /// Import vault items from CSV, JSON, Bitwarden, or KeePass
    Import {
        file: String,
    },

    /// Export vault items to CSV, JSON, or standalone offline HTML decryptor
    Export {
        file: String,
    },

    /// Generate shell auto-completions for zsh, bash, fish, powershell
    Completions {
        shell: Shell,
    },

    /// Display current vault cryptographic status
    Status,

    /// Show version and build identity
    Version,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None | Some(Commands::Tui) => {
            let items = vault::database::load_items();
            tui::run_tui(items)?;
        }
        Some(Commands::List { json, category }) => {
            commands::list::execute(json, category);
        }
        Some(Commands::Get { name, password, username, totp, copy, json }) => {
            commands::get::execute(&name, password, username, totp, copy, json);
        }
        Some(Commands::Add { title, username, password, note, category }) => {
            commands::add::execute(title, username, password, note, category)?;
        }
        Some(Commands::Remove { name }) | Some(Commands::Delete { name }) => {
            commands::remove::execute(name)?;
        }
        Some(Commands::Search { query }) => {
            commands::search::execute(&query);
        }
        Some(Commands::Totp { name, watch, copy }) => {
            commands::totp::execute(&name, watch, copy);
        }
        Some(Commands::Generate { length, diceware }) => {
            let pass = commands::generate::execute(length, diceware);
            println!("{}", pass);
        }
        Some(Commands::Run { command }) => {
            let items = vault::database::load_items();
            commands::run::execute(&items, &command)?;
        }
        Some(Commands::Audit { json }) => {
            let items = vault::database::load_items();
            commands::audit::execute(&items, json);
        }
        Some(Commands::Sss { split, recover }) => {
            if let Some(shards) = recover {
                commands::sss::recover(shards);
            } else if split {
                commands::sss::split(None);
            } else {
                commands::sss::split(None);
            }
        }
        Some(Commands::Orvsend { text, expires, passphrase }) => {
            commands::orvsend::create(&text, expires, passphrase);
        }
        Some(Commands::Ssh { action }) => {
            let items = vault::database::load_items();
            commands::ssh::execute(&items, action);
        }
        Some(Commands::Import { file }) => {
            commands::import::execute(&file);
        }
        Some(Commands::Export { file }) => {
            commands::export::execute(&file);
        }
        Some(Commands::Completions { shell }) => {
            commands::completions::execute(shell);
        }
        Some(Commands::Status) => {
            commands::status::execute();
        }
        Some(Commands::Version) => {
            println!("Orvpass v5.0.0 Enterprise (Pure Rust CLI & TUI)");
            println!("Engine: Argon2id + ChaCha20-Poly1305 AEAD");
        }
    }

    Ok(())
}
