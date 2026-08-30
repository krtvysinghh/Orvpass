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
        #[arg(long, default_value_t = 15)]
        copy_timeout: u64,
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
        #[arg(long, default_value_t = 15)]
        copy_timeout: u64,
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

    /// Synchronize or export .env configuration files
    Dotenv {
        #[arg(short, long, default_value = ".env")]
        file: String,
        #[arg(short, long)]
        export: bool,
    },

    /// Inject secrets dynamically into Docker compose
    Docker {
        service: String,
    },

    /// Generate Kubernetes Secret manifest from vault items
    K8s {
        #[arg(short, long, default_value = "default")]
        namespace: String,
    },

    /// Output Terraform provider schema
    Tf,

    /// Generate AWS STS credentials JSON for AWS CLI
    Aws {
        #[arg(short, long, default_value = "default")]
        profile: String,
    },

    /// Git credential helper bridge
    Git {
        action: String,
    },

    /// UNIX pipeline raw output without trailing newlines
    Pipe {
        name: String,
        #[arg(short, long, default_value = "password")]
        field: String,
    },

    /// FIDO2 / YubiKey hardware authentication challenge
    Yubikey,

    /// Hardware Secure Enclave / TPM 2.0 status
    SecureEnclave,

    /// Coercion panic multi-pass memory & disk wipe
    DuressWipe,

    /// Post-Quantum ML-KEM-768 hybrid key encapsulation
    PqcKem,

    /// Native age encryption recipient plugin
    Age,

    /// Digital will & dead man's switch countdown
    DeadManSwitch {
        #[arg(short, long)]
        arm: bool,
    },

    /// Benchmark Argon2id and ChaCha20 performance
    Bench,

    /// Render terminal ANSI QR Code for mobile 2FA scanning
    Qr {
        name: String,
    },

    /// Offline Have I Been Pwned k-anonymity breach scan
    PwnedCheck,

    /// TLS certificate and SSH key expiration monitor
    CertExpiry,

    /// Corporate password policy compliance check
    Policy,

    /// 1-click credential auto-rotation
    Rotate {
        name: String,
    },

    /// Export compliance audit report (SOC2 / ISO-27001)
    AuditExport {
        #[arg(short, long, default_value = "md")]
        format: String,
    },

    /// Generate Git pre-commit secret leak detector hook
    LeakDetector,

    /// Telemetry and access anomaly detector
    AnomalousLog,

    /// Multi-tenant role-based team vault partitions
    OrgVault {
        #[arg(short, long, default_value = "list")]
        action: String,
    },

    /// Peer-to-peer LAN / Tailscale vault sync
    P2pSync {
        peer: String,
    },

    /// Dispatch HMAC-signed mutation webhook
    Webhook {
        url: String,
    },

    /// Privacy email forwarding alias generator
    AliasDns {
        #[arg(short, long, default_value = "privacy.dev")]
        domain: String,
    },

    /// Session key caching daemon
    Daemon {
        #[arg(short, long, default_value = "status")]
        action: String,
    },

    /// Multi-signature M-of-N governance quorum
    MultiSig {
        #[arg(short, long, default_value = "status")]
        action: String,
    },

    /// Output shell integration script for fzf
    Fzf,

    /// Output tmux status line widget
    TmuxStatus,

    /// Output quick shell wrapper functions (op, opg, opl)
    AliasWrapper {
        #[arg(short, long, default_value = "zsh")]
        shell: String,
    },

    /// Generate man pages for UNIX man documentation
    Man,

    /// Password entropy and crack-time analyzer
    Strength {
        password: String,
    },

    /// System, RNG, and compiler diagnostics
    Doctor,

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
        Some(Commands::Get { name, password, username, totp, copy, json, .. }) => {
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
        Some(Commands::Totp { name, watch, copy, .. }) => {
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
        Some(Commands::Dotenv { file, export }) => {
            let items = vault::database::load_items();
            commands::devtools::dotenv_sync(&items, &file, export)?;
        }
        Some(Commands::Docker { service }) => {
            let items = vault::database::load_items();
            commands::devtools::docker_secret(&items, &service);
        }
        Some(Commands::K8s { namespace }) => {
            let items = vault::database::load_items();
            commands::devtools::k8s_sync(&items, &namespace);
        }
        Some(Commands::Tf) => {
            commands::devtools::tf_provider();
        }
        Some(Commands::Aws { profile }) => {
            let items = vault::database::load_items();
            commands::devtools::aws_vault(&profile, &items);
        }
        Some(Commands::Git { action }) => {
            let items = vault::database::load_items();
            commands::devtools::git_credential(&action, &items);
        }
        Some(Commands::Pipe { name, field }) => {
            let items = vault::database::load_items();
            commands::devtools::pipe(&items, &name, &field);
        }
        Some(Commands::Yubikey) => {
            commands::security_ext::yubikey_challenge();
        }
        Some(Commands::SecureEnclave) => {
            commands::security_ext::secure_enclave_status();
        }
        Some(Commands::DuressWipe) => {
            commands::security_ext::duress_wipe();
        }
        Some(Commands::PqcKem) => {
            commands::security_ext::pqc_kem();
        }
        Some(Commands::Age) => {
            commands::security_ext::age_plugin();
        }
        Some(Commands::DeadManSwitch { arm }) => {
            commands::security_ext::dead_man_switch(arm);
        }
        Some(Commands::Bench) => {
            commands::security_ext::benchmark();
        }
        Some(Commands::Qr { name }) => {
            commands::security_ext::render_qr(&name);
        }
        Some(Commands::PwnedCheck) => {
            let items = vault::database::load_items();
            commands::security_ext::pwned_check(&items);
        }
        Some(Commands::CertExpiry) => {
            let items = vault::database::load_items();
            commands::security_ext::cert_expiry(&items);
        }
        Some(Commands::Policy) => {
            let items = vault::database::load_items();
            commands::security_ext::policy_check(&items);
        }
        Some(Commands::Rotate { name }) => {
            commands::security_ext::auto_rotate(&name);
        }
        Some(Commands::AuditExport { format }) => {
            let items = vault::database::load_items();
            commands::security_ext::audit_export(&format, &items);
        }
        Some(Commands::LeakDetector) => {
            commands::security_ext::leak_detector_hook();
        }
        Some(Commands::AnomalousLog) => {
            commands::security_ext::anomalous_log();
        }
        Some(Commands::OrgVault { action }) => {
            commands::security_ext::org_vault(&action);
        }
        Some(Commands::P2pSync { peer }) => {
            commands::security_ext::p2p_sync(&peer);
        }
        Some(Commands::Webhook { url }) => {
            commands::security_ext::webhook(&url);
        }
        Some(Commands::AliasDns { domain }) => {
            commands::security_ext::alias_dns(&domain);
        }
        Some(Commands::Daemon { action }) => {
            commands::security_ext::daemon(&action);
        }
        Some(Commands::MultiSig { action }) => {
            commands::security_ext::multi_sig(&action);
        }
        Some(Commands::Fzf) => {
            commands::security_ext::fzf_script();
        }
        Some(Commands::TmuxStatus) => {
            commands::security_ext::tmux_status();
        }
        Some(Commands::AliasWrapper { shell }) => {
            commands::security_ext::alias_wrapper(&shell);
        }
        Some(Commands::Man) => {
            commands::security_ext::man_pages();
        }
        Some(Commands::Strength { password }) => {
            commands::security_ext::strength_meter(&password);
        }
        Some(Commands::Doctor) => {
            commands::security_ext::doctor();
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
