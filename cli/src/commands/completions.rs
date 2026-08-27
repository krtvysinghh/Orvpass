use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;

pub fn execute(shell: Shell) {
    let mut cmd = crate::Cli::command();
    generate(shell, &mut cmd, "orvpass", &mut io::stdout());
}
