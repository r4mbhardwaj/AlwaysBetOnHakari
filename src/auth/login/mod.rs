pub fn login(_username: &str, _password: &str) -> bool {
    // check if the username and password are correct
    println!("Checking if the username and password are correct");
    true // return true for now
}

pub fn generate_jwt(username: &str) -> String {
    // generate a jwt for the user
    println!("Generating a jwt for the user");
    username.to_string() // return the username for now
}

pub fn start() -> String {
    println!("-x-x-x-x-x-x-");
    println!("Login");
    println!("-x-x-x-x-x-x-");

    let mut username = String::new();
    let mut password = String::new();

    println!("Username: ");
    std::io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    println!("Password: ");
    std::io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    let username = username.trim();
    let password = password.trim();

    if username.is_empty() || password.is_empty() {
        println!("Username or password is empty");
        return "".to_string();
    }

    if !login(username, password) {
        println!("Failed to login");
        return "".to_string();
    }

    println!("Logged in successfully");

    generate_jwt(username)
}
