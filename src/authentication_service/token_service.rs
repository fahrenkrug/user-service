use std::{env};
use std::fmt::{Display, Formatter};
use branca::Branca;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{Duration, Utc};

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

pub fn user_id_from(token: &str) -> Result<String, DecodingError> {
    Ok(decode_token(token)?.sub)
}

fn decode_token(token: &str) -> Result<Claims, DecodingError> {
    let tokenizer = tokenizer();
    match tokenizer.decode(token, 0) {
        Ok(token) => serialize_token(&token),
        Err(e) => {
            print!("branca error: {}", e);
            Err(DecodingError)
        }
    }
}

fn serialize_token(token: &[u8]) -> Result<Claims, DecodingError> {
    match rmp_serde::from_slice::<Claims>(token) {
        Ok(claims) => Ok(claims),
        Err(e) => {
            println!("serialize error: {}", e);
            Err(DecodingError)
        }
    }
}

#[derive(Clone, Copy)]
pub struct DecodingError;

impl Display for DecodingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid byte string token to decode")
    }
}