use std::env;
use branca::Branca;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{Duration, Utc};
use rmp_serde::{Serializer};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,         // Optional. Audience
    exp: i64,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: i64,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    sub: String,         // Optional. Subject (whom token refers to)
}

impl Claims {
    fn new(user_id: &Uuid) -> Claims {
        let aud = format!("{}-{}", env::var("AUDIENCE").unwrap_or_default(),"authentication-service");
        Claims {
            aud,
            iss: String::from("authentication-service"),
            exp: (Utc::now() + Duration::days(14)).timestamp(),
            iat: Utc::now().timestamp(),
            sub: user_id.to_string(),
        }
    }
}

fn tokenizer() -> Branca {
    let key = env::var("TOKEN_SALT").expect("salt must be set");
    Branca::new(key.as_bytes()).expect("branca initilization failed")
}

pub fn create_token(user_id: &Uuid) -> String {
    let mut tokenizer = tokenizer();
    let claims = Claims::new(user_id);
    let bytes = rmp_serde::to_vec(&claims).unwrap();
    tokenizer.encode(&bytes).unwrap()
}