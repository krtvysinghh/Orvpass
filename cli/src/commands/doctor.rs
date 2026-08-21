use std::path::PathBuf;


pub fn run(){

let vault =
PathBuf::from(std::env::var("HOME").unwrap())
.join(".orvpass")
.join("vault.orv");


println!("Orvpass Doctor");
println!("Vault: {}", vault.exists());

match std::env::current_dir(){

Ok(p)=>println!("Project: {:?}",p),

Err(_)=>println!("Project unavailable")

}

}
