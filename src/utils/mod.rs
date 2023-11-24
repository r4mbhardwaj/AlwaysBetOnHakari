use std::io::{self, Write}; // Import the Write trait

pub fn input(prompt: &str) -> io::Result<String> {
    // Output the prompt
    print!("{}", prompt);
    // Because the prompt isn't ended with a newline, we need to flush stdout
    // or else the prompt doesn't appear immediately on the terminal.
    io::stdout().flush()?;

    // Now get the user's input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Return the input string
    Ok(input.trim().to_string())
}
