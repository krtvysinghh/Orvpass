use std::path::PathBuf;


pub fn run(){

let vault =
PathBuf::from(
std::env::var("HOME").unwrap()
)
.join(".orvpass")
.join("vault.orv");


println!("Orvpass Health");

if vault.exists(){

println!("Vault: OK");

}else{

println!("Vault: Missing");

}

}

