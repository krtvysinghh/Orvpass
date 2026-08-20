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
        Commands::Init => real_init(),
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



use dialoguer::Password;
use std::path::PathBuf;
use orvpass_core::crypto::SecretKey;
use orvpass_core::vault::Vault;





fn real_init() {
    let _password = Password::new()
        .with_prompt("Create master password")
        .interact()
        .unwrap();

    let key = SecretKey::generate();

    let vault_dir = PathBuf::from(std::env::var("HOME").unwrap())
        .join(".orvpass");

    std::fs::create_dir_all(&vault_dir)
        .expect("cannot create vault directory");

    let vault_file = vault_dir.join("vault.orv");

    if vault_file.exists() {
        if vault_file.is_dir() {
            std::fs::remove_dir_all(&vault_file)
                .expect("cannot remove old vault directory");
        } else {
            std::fs::remove_file(&vault_file)
                .expect("cannot remove old vault file");
        }
    }

    let mut vault = Vault::new_locked_at(&vault_file);

    vault.initialize(&key)
        .expect("vault initialization failed");

    println!("Vault initialized at {:?}", vault_file);
}


