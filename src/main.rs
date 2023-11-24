mod auth;

use auth::login;

mod game {

    pub mod stats {
        // for checking the stats of the player
        pub fn stats() {
            println!("Checking the stats");
        }
    }

    pub mod play {
        // for playing the game
        pub fn play() {
            println!("Playing the game");
        }
    }

    pub fn is_authenticated(_jwt: &str) -> bool {
        // check if the jwt is valid
        println!("Checking if the user is authenticated");
        true // return true for now
    }

    // start the game
    pub fn start(jwt: &str) {
        println!("====================");
        println!("Starting the game");
        println!("JWT: {}", jwt);

        if !is_authenticated(jwt) {
            println!("User is not authenticated");
            return;
        }

        println!("User is authenticated");

        let mut exit = false;

        while !exit {
            println!("-x-x-x-x-x-");
            println!("1. Play");
            println!("2. Stats");
            println!("3. Exit");

            let mut choice = String::new();
            std::io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

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
}

fn main() {
    println!("Hello, world!");
    println!("This is the main.rs file");
    let jwt = auth::start();
    if jwt == "" {
        println!("Exiting the Auth");
        return;
    }
    game::start(&jwt);
}
