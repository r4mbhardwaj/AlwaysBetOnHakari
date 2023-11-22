mod auth;

use auth::{jwt, login, register};

fn main() {
    println!("Hello, world!");
    println!("This is the main.rs file");
    println!("{:?}", login::login("Ram", "rambhardwaj"));
}
