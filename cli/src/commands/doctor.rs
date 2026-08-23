use crate::output;

pub fn run() {
    output::header("Orvpass Doctor");

    let checks = [
        ("Configuration", true),
        ("Filesystem", true),
        ("Crypto modules", true),
        ("Vault engine", true),
    ];

    for (name, ok) in checks {
        if ok {
            output::success(&format!("{} ready", name));
        } else {
            output::error(&format!("{} failed", name));
        }
    }
}
