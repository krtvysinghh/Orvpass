use orvpass_core::models::{ItemData, VaultItem};
use std::process::Command;

pub fn execute(items: &[VaultItem], command_args: &[String]) -> anyhow::Result<()> {
    if command_args.is_empty() {
        println!("❌ Error: No command specified. Usage: orvpass run -- <command> [args...]");
        return Ok(());
    }

    let program = &command_args[0];
    let args = &command_args[1..];

    let mut cmd = Command::new(program);
    cmd.args(args);

    // Inject decrypted secrets into child process environment
    for item in items {
        if let ItemData::Login(login) = &item.data {
            let env_key = item.title.to_uppercase().replace([' ', '-', '.'], "_");
            if let Some(user) = &login.username {
                cmd.env(format!("{}_USER", env_key), user);
            }
            if let Some(pass) = &login.password {
                cmd.env(format!("{}_PASSWORD", env_key), pass);
                cmd.env(format!("{}_SECRET", env_key), pass);
            }
        }
        for cf in &item.custom_fields {
            let cf_key = format!("{}_{}", item.title.to_uppercase(), cf.name.to_uppercase()).replace([' ', '-', '.'], "_");
            cmd.env(cf_key, &cf.value);
        }
    }

    let status = cmd.status()?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}
