#!/bin/bash
set -e

echo "=== FIXING BATCH 1 ERRORS ==="

# add rand dependency if missing
if ! grep -q '^rand' cli/Cargo.toml 2>/dev/null; then
    cargo add rand --manifest-path cli/Cargo.toml
fi


python3 - <<'PY'
from pathlib import Path

p = Path("cli/src/main.rs")
s = p.read_text()

old = "Commands::Get => commands::get::run(),"

new = """
Commands::Get => {
    use std::io::{self, Write};

    print!("Item ID: ");
    io::stdout().flush().unwrap();

    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();

    commands::get::run(id.trim().to_string())
},
"""

if old in s:
    s=s.replace(old,new)

p.write_text(s)
PY


cargo fmt --all

cargo test --workspace

echo "=== FIX COMPLETE ==="
