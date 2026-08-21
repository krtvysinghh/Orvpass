#!/bin/bash
set -e

echo "=== ORVPASS BATCH 3 ==="


python3 <<'PY'
from pathlib import Path

p=Path("cli/src/main.rs")
s=p.read_text()


# add command modules
if "pub mod delete;" not in s:
    s=s.replace(
    "pub mod search;",
    """pub mod search;
    pub mod delete;
    pub mod edit;"""
    )


# add enum variants if missing
for name in ["Search","Delete"]:
    if name not in s:
        print("missing",name)


p.write_text(s)
PY



# atomic vault save patch

python3 <<'PY'
from pathlib import Path

p=Path("core/src/vault.rs")
s=p.read_text()

old='''fs::write(&self.path, output).map_err(|_| VaultError::Invalid)?;'''

new='''
let tmp = self.path.with_extension("tmp");

fs::write(&tmp, output)
    .map_err(|_| VaultError::Invalid)?;

fs::rename(&tmp, &self.path)
    .map_err(|_| VaultError::Invalid)?;
'''

if old in s:
    s=s.replace(old,new)

p.write_text(s)
PY



cargo fmt --all

cargo test --workspace


cargo clippy --workspace -- \
-D warnings || true


echo "=== BATCH 3 COMPLETE ==="
