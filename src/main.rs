mod auth;
mod utils;

mod game {
    use crate::utils::input;
    pub mod stats {
        // for checking the stats of the player
        pub fn stats() {
            println!("Checking the stats");
        }
    }

    pub mod play {
        use crate::utils::input;
        use rand::Rng;

        // for playing the game
        pub fn play() {
            println!("Playing the game");

            // make user choose between 1-10
            let mut choice: i8 = 0;
            while !(1..=10).contains(&choice) {
                println!("Choose a number between 1-10");
                let input = input("> ").expect("Failed to read line");
                choice = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
            }

            let random_number = rand::thread_rng().gen_range(1..11); // generate a random number between 1-10
            println!("==================================");
            println!("|\t Your choice:\t{}\t|", choice);
            println!("|\t Random number:\t{}\t|", random_number);
            println!("==================================");

            // check if the user guessed the correct number
            if choice == random_number {
                println!("You guessed the correct number");
            } else {
                println!("You guessed the wrong number");
            }
        }
    }

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
}

fn main() {
    println!("Welcome to CursedRoll!");
    let jwt = auth::start();
    if jwt.is_empty() {
        println!("Exiting the Auth");
        return;
    }
    game::start(&jwt);
}
