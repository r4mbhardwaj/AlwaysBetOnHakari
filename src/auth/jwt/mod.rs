const SECRET_KEY: &str = "ajsfgasdfasdklfnajsdhf";

pub fn create_token(username: &str, password: &str) -> String {
    println!("Creating token for user: {}", username);
    println!("Secret key: {}", SECRET_KEY);

    "hello world".to_string()
}
