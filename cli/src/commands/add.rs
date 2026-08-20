use dialoguer::Input;

pub fn run() {
    let name: String = Input::new()
        .with_prompt("Service")
        .interact_text()
        .unwrap();

    println!("Added credential for {}", name);
}
