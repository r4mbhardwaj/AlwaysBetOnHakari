pub const LOGIN_FIELDS: [&str; 2] = ["username", "password"];

pub mod jwt;
pub mod login;
pub mod register;

pub fn start() -> String {
    // ask the user to login or register
    println!("-x-x-x-x-x-");
    println!("Auth");
    println!("1. Login");
    println!("2. Register");
    println!("3. Exit");

    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => return "".to_string(),
    };

    match choice {
        1 => return login::start(),
        2 => return register::start(),
        _ => "".to_string(),
    }
}
