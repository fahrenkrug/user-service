use argon2::{self, Config};
use diesel::RunQueryDsl;
use rand::distributions::Standard;
use rand::Rng;
use uuid::Uuid;
use crate::models::email_identities::NewEmailIdentity;
use crate::Pool;

pub struct EmailIdentityService {
    config: Config<'static >,
}

impl EmailIdentityService {
    pub fn new() -> EmailIdentityService {
        EmailIdentityService {
            config: Config::default(),
        }
    }

    fn encode(&self, password: &str) -> String {
        let salt = EmailIdentityService::generate_random_salt();
        argon2::hash_encoded(password.as_bytes(), &*salt, &self.config).unwrap()
    }

    pub fn matches(input: &str, stored_password_hash: String) -> bool{
        match argon2::verify_encoded(&stored_password_hash, input.as_bytes()) {
            Ok(is_match ) => is_match,
            Err(e) => {
                println!("Password error during verify: {}", e);
                false
            }
        }
    }

    pub async fn store_email_identity(&self, pool: &Pool, password: &str, user_id: &Uuid, email: &str) {
        use super::super::schema::email_identities;
        println!("Passowrd is {}", &password);
        let hashed = self.encode(password);
        println!("Hashed pw is {}", &hashed );
        let new_email_identity = NewEmailIdentity::new(user_id, hashed, email);
        let connection = pool.get().unwrap();
        diesel::insert_into(email_identities::table).values(&new_email_identity).execute(&connection).expect("Error storing password");
        println!("Stored password")
    }

    fn generate_random_salt() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let length_of_salt: usize = rng.gen_range(64..255);
        (&mut rng).sample_iter(Standard).take(length_of_salt).collect()
    }
}
