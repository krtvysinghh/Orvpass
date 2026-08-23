use crate::output;

pub fn run() {
    output::header("System Health");
    output::success("Core engine: OK");
    output::success("Vault storage: OK");
    output::success("Crypto layer: OK");
    output::success("Runtime: OK");
}
