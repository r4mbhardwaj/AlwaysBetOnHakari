mod auth;

use auth::login;

fn main() {
    println!("Hello, world!");
    println!("This is the main.rs file");
    println!("{:?}", login::login("Ram", "rambhardwaj"));
}
