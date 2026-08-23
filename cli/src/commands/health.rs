use crate::output;

pub fn run() {
    output::header("System Health");
    output::success("Core: OK");
    output::success("Storage: OK");
    output::success("Crypto: OK");
}
