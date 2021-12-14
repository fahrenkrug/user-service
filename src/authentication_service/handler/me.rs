use uuid::Uuid;
use tonic::{Request, Response, Status};
use crate::Pool;
use super::super::token_service;
use super::super::{UserRequest, UserResponse};
use crate::models::users::User;
use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

type MeReply = Result<Response<UserResponse>, Status>;

pub fn respond(pool: &Pool, request: Request<UserRequest>) -> MeReply {
    match request.metadata().get("token") {
        Some(token) => {
            match token.to_str() {
                Err(e) => {
                    println!("Error transforming metadata into string: ${}", e);
                    unauthenticated()
                },
                Ok(token) => get_user_for_token(pool, token)
            }
        },
        None => unauthenticated()
    }
}

fn unauthenticated() -> MeReply {
    Err(Status::unauthenticated("Credentials don't match or user does not exist."))
}

fn get_user_for_token(pool: &Pool, token: &str) -> MeReply {
    match token_service::user_id_from(token) {
        Err(e) => {
            println!("error getting user from token: {}", e);
            unauthenticated()
        },
        Ok(user_id) => {
            use crate::schema::users::dsl::*;
            let connection = pool.get().unwrap();
            match Uuid::parse_str(&user_id) {
                Err(e) => {
                    println!("Error parsing token to uuid: {}", e);
                    unauthenticated()
                },
                Ok(token) => {
                    match users.filter(id.eq(token)).first::<User>(&connection) {
                        Ok(user) => response_from_user(&user),
                        Err(e) => {
                            println!("error loading user from database: {}", e);
                            unauthenticated()
                        }
                    }
                }
            }
        }
    }
}

fn response_from_user(user: &User) -> MeReply {
    let response = UserResponse {
        id: user.id.to_string(),
        first_name: user.first_name.to_owned(),
        last_name: user.last_name.to_owned(),
        created_at: user.created_at.to_string(),
        updated_at: user.updated_at.to_string(),
    };
    Ok(Response::new(response))
}
