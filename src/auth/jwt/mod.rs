const SECRET_KEY: &str = "ajsfgasdfasdklfnajsdhf";

pub fn create_token(_username: &str) -> String {
    println!("Creating token for user: {}", _username);
    println!("Secret key: {}", SECRET_KEY);

    "hello world".to_string()
}
