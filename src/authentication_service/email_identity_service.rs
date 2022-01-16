use argon2::{self, Config};
use diesel::dsl::exists;
use rand::distributions::Standard;
use rand::Rng;
use uuid::Uuid;
use crate::models::email_identities::NewEmailIdentity;
use crate::{Pool, PooledConnection};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use tonic::Status;
use super::token_service;

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

    pub fn matches_user_id(connection: &PooledConnection, input: &str, user_uuid: &str) -> Result<(bool, String), Status> {
        use crate::schema::email_identities::dsl::*;
        match Uuid::parse_str(user_uuid) {
            Ok(uuid) => {
                println!("Looking for user with uuid: {}", uuid.to_string());
                match email_identities.filter((user_id).eq(uuid)).filter(created_at.eq(updated_at)).order(created_at.desc()).select((hash, email)).first::<(String, String)>(connection) {
                    Ok((password_hash, user_email)) => Ok((EmailIdentityService::matches(input, password_hash), user_email)),
                    Err(e) => {
                        println!("error while trying to find user with id: {}", e);
                        Err(Status::not_found("User with user_id not found"))
                    }
                }
            },
            Err(e) => {
                println!("Error parsing uuid: {}", e);
                Err(token_service::unauthenticated_plain())
            }
        }
    }

    pub async fn store_email_identity(&self, pool: &Pool, password: &str, user_id: &Uuid, email: &str) {
        use super::super::schema::email_identities;
        let hashed = self.encode(password);
        let new_email_identity = NewEmailIdentity::new(user_id, hashed, email);
        let connection = pool.get().unwrap();
        diesel::insert_into(email_identities::table).values(&new_email_identity).execute(&connection).expect("Error storing password");
    }

    pub async fn does_email_exist(&self, pool: &Pool, user_email: &str) -> bool {
        use super::super::schema::email_identities::dsl::*;
        let connection = pool.get().unwrap();
        diesel::select(exists(email_identities.filter(email.eq(user_email)))).get_result(&connection).expect("Error accessing email identity")
    }

    fn generate_random_salt() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let length_of_salt: usize = rng.gen_range(64..255);
        (&mut rng).sample_iter(Standard).take(length_of_salt).collect()
    }
}
