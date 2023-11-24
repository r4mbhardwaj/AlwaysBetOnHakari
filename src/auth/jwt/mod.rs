use jwt_simple::prelude::*;
use std::collections::HashMap;

pub const SECRET_KEY: &str = "your-secret-key-here";

pub fn create_token(_username: &str, _role: &str) -> Result<String, jwt_simple::Error> {
    println!("Creating token for user: {}", _username);
    println!("Secret key: {:?}", SECRET_KEY);

    let mut custom_claims = HashMap::new();
    custom_claims.insert("role".to_string(), _role.to_string());
    custom_claims.insert("login_time".to_string(), chrono::Utc::now().to_rfc3339());

    let claims = Claims::with_custom_claims(custom_claims, Duration::from_hours(1));
    let claims = claims.with_subject(_username);

    let key = HS256Key::from_bytes(SECRET_KEY.as_bytes());

    let jwt = key.authenticate(claims)?;

    Ok(jwt)
}
