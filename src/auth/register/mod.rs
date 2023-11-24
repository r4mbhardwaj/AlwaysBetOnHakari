pub fn register(_username: &str, _password: &str) -> bool {
    true
}

pub fn start() -> String {
    println!("-x-x-x-x-x-x-");
    println!("Register");
    println!("-x-x-x-x-x-x-");
    println!("Username: ");
    let mut username = String::new();

    std::io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    println!("Password: ");
    let mut password = String::new();

    std::io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    let username = username.trim();
    let password = password.trim();

    if username.is_empty() || password.is_empty() {
        println!("Username or password is empty");
        return "".to_string();
    }

    if !register(username, password) {
        println!("Failed to register");
        return "".to_string();
    }

    println!("Registered successfully");

    crate::auth::login::generate_jwt(username)
}
