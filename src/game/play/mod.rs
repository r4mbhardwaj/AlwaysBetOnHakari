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
