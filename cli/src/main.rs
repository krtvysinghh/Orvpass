use clap::{Parser,Subcommand};

#[derive(Parser)]
#[command(name="orvpass")]
#[command(version="2.0.0")]

struct App{
    #[command(subcommand)]
    command:Option<Command>
}

#[derive(Subcommand)]
enum Command{
    Init,
    Add,
    List,
    Unlock,
    Status,
    Generate,
    Audit,
}

fn main(){

let app=App::parse();

match app.command{

Some(Command::Status)=>
println!("ORVPASS v2.0.0 SECURE CORE ONLINE"),

Some(Command::Audit)=>
println!("Security audit ready"),

_=>println!("Orvpass Password Manager v2.0.0")
}

}
