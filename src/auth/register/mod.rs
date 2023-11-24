use crate::utils::input;

pub fn register(_username: &str, _password: &str) -> bool {
    true
}

pub fn start() -> String {
    println!("-x-x-x-x-x-x-");
    println!("Register");
    println!("-x-x-x-x-x-x-");
    let username = input("Username: ").expect("Failed to read line");
    let password = input("Password: ").expect("Failed to read line");

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
