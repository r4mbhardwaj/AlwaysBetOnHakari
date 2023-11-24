mod auth;
mod utils;

mod game;

fn main() {
    println!("Welcome to CursedRoll!");
    let jwt = auth::start();
    if jwt.is_empty() {
        println!("Exiting the Auth");
        return;
    }
    game::start(&jwt);
}
