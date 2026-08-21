#!/bin/bash
set -e

echo "=== ORVPASS BATCH 2 ==="


# -------------------------
# FIX RAND API
# -------------------------

cat > cli/src/commands/generate.rs <<'RS'
use rand::Rng;

pub fn run() {

    let chars =
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*";

    let mut rng = rand::rng();

    let password:String =
        (0..32)
        .map(|_| {
            let i = rng.random_range(0..chars.len());
            chars[i] as char
        })
        .collect();

    println!("{}",password);
}
RS



# -------------------------
# SEARCH COMMAND
# -------------------------

cat > cli/src/commands/search.rs <<'RS'
use dialoguer::Password;
use orvpass_core::crypto::SecretKey;
use orvpass_core::vault::Vault;
use std::path::PathBuf;


pub fn run(query:String){

let master =
Password::new()
.with_prompt("Master password")
.interact()
.unwrap();


let key =
SecretKey::from_password(&master)
.unwrap();


let path =
PathBuf::from(std::env::var("HOME").unwrap())
.join(".orvpass")
.join("vault.orv");


let mut vault =
Vault::new(path);


vault.unlock(&key)
.expect("unlock failed");


for item in vault.items(){

if item.title
.to_lowercase()
.contains(&query.to_lowercase())
{
println!("{} [{:?}]",
item.title,
item.item_type);
}

}

}
RS



# -------------------------
# DELETE COMMAND
# -------------------------

cat > cli/src/commands/delete.rs <<'RS'
use dialoguer::Password;
use orvpass_core::crypto::SecretKey;
use orvpass_core::vault::Vault;
use std::path::PathBuf;


pub fn run(id:String){

let master =
Password::new()
.with_prompt("Master password")
.interact()
.unwrap();


let key =
SecretKey::from_password(&master)
.unwrap();


let path =
PathBuf::from(std::env::var("HOME").unwrap())
.join(".orvpass")
.join("vault.orv");


let mut vault =
Vault::new(path);


vault.unlock(&key)
.expect("unlock failed");


vault.remove(id)
.expect("delete failed");


vault.save(&key)
.expect("save failed");


println!("✓ Deleted");

}
RS



# -------------------------
# REGISTER MODULE
# -------------------------

python3 <<'PY'
from pathlib import Path

p=Path("cli/src/main.rs")
s=p.read_text()

if "mod search;" not in s:
    s=s.replace(
        "mod commands {",
        """mod commands {
    pub mod search;
""")

if "Search" not in s:
    pass

p.write_text(s)

PY


cargo fmt --all

cargo test --workspace


echo "=== BATCH 2 COMPLETE ==="
