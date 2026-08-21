use dialoguer::Password;
use orvpass_core::crypto::SecretKey;
use orvpass_core::vault::Vault;
use std::path::PathBuf;


pub fn run(query:String){

let master=Password::new()
.with_prompt("Master password")
.interact()
.unwrap();


let key=SecretKey::from_password(&master)
.unwrap();


let path=PathBuf::from(
std::env::var("HOME").unwrap()
)
.join(".orvpass")
.join("vault.orv");


let mut vault=Vault::new(path);


match vault.unlock(&key){

Ok(_)=>{

let q=query.to_lowercase();

let mut found=false;


for item in vault.items(){

if item.title.to_lowercase().contains(&q){

println!(
"- {} [{:?}]",
item.title,
item.item_type
);

found=true;

}

}


if !found{

println!("No matching items");

}

}

Err(e)=>println!("Unlock failed: {}",e)

}

}

