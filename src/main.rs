mod auth;

use auth::login;

mod Game {
    use crate::auth::jwt;

    pub mod Stats {
        // for checking the stats of the player
        pub fn stats() {
            println!("Checking the stats");
        }
    }

    pub mod Play {
        // for playing the game
        pub fn play() {
            println!("Playing the game");
        }
    }

    pub fn is_authenticated(jwt: &str) -> bool {
        // check if the jwt is valid
        println!("Checking if the user is authenticated");
        true // return true for now
    }

    // start the game
    pub fn start(jwt: &str) {
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
                1 => Play::play(),
                2 => Stats::stats(),
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
    // println!("{:?}", login::login("Ram", "rambhardwaj"));
    Game::start("adfasfjkhsdfasgdfhsfj");
}
