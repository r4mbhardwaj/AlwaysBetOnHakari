use std::collections::HashMap;

use crate::utils::input;

pub fn login(_username: &str, _password: &str) -> bool {
    // check if the username and password are correct
    println!("Checking if the username and password are correct");
    true // return true for now
}

pub fn generate_jwt(username: &str) -> String {
    // generate a jwt for the user
    println!("Generating a jwt for the user");
    crate::auth::jwt::create_token(username)
}

pub fn start() -> String {
    println!("-x-x-x-x-x-x-");
    println!("Login");
    println!("-x-x-x-x-x-x-");

    let credentials = super::LOGIN_FIELDS
        .iter()
        .map(|field| {
            let value = input(&format!("{}: ", field)).expect("Failed to read line");
            (field.to_string(), value)
        })
        .collect::<HashMap<String, String>>();

    println!("{:?}", credentials);
    println!("{:?}", credentials.get("username").unwrap());

    if credentials.get("username").unwrap().is_empty()
        || credentials.get("password").unwrap().is_empty()
    {
        println!("Username or password is empty");
        return "".to_string();
    }

    if !login(
        credentials.get("username").unwrap(),
        credentials.get("password").unwrap(),
    ) {
        println!("Failed to login");
        return "".to_string();
    }

    println!("Logged in successfully");

    generate_jwt(credentials.get("username").unwrap())
}
