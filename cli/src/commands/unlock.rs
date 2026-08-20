use dialoguer::Password;

pub fn run() {
    let _password = Password::new()
        .with_prompt("Master password")
        .interact()
        .unwrap();

    println!("Vault unlocked");
}
