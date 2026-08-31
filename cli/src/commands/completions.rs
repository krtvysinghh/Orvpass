use clap::CommandFactory;
use clap_complete::{Shell, generate};
use std::io;

pub fn execute(shell: Shell) {
    let mut cmd = crate::Cli::command();
    generate(shell, &mut cmd, "orvpass", &mut io::stdout());
}
