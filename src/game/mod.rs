use crate::utils::input;
pub mod play;
pub mod stats;

pub fn is_authenticated(_jwt: &str) -> bool {
    // check if the jwt is valid
    println!("Checking if the user is authenticated");
    true // return true for now
}

// start the game
pub fn start(jwt: &str) {
    println!("\n\n====================");
    println!("Starting the game");
    println!("JWT: {}", jwt);

    if !is_authenticated(jwt) {
        println!("User is not authenticated");
        return;
    }

    println!("User is authenticated");

    let mut exit = false;

    while !exit {
        println!("\n-x-x-x-x-x-");
        println!("1. Play");
        println!("2. Stats");
        println!("3. Exit");

        let choice = input("> ").expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => play::play(),
            2 => stats::stats(),
            3 => exit = true,
            _ => continue,
        }
    }
    println!("Exiting the game");
}
