use uuid::Uuid;
use tonic::{Request, Response, Status};
use crate::Pool;
use super::super::token_service;
use super::super::{UserRequest, UserResponse};
use crate::models::users::User;
use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

type MeReply = Result<Response<UserResponse>, Status>;

pub fn respond(pool: &Pool, request: Request<UserRequest>) -> MeReply {
    token_service::verify(request, |_, user_id| get_user_for_user_id(pool, &user_id))
}

fn get_user_for_user_id(pool: &Pool, user_id: &str) -> MeReply {
    use crate::schema::users::dsl::*;
    let connection = pool.get().unwrap();
    match Uuid::parse_str(user_id) {
        Err(e) => {
            println!("Error parsing token to uuid: {}", e);
            token_service::unauthenticated()
        },
        Ok(token) => {
            match users.filter(id.eq(token)).first::<User>(&connection) {
                Ok(user) => response_from_user(&user),
                Err(e) => {
                    println!("error loading user from database: {}", e);
                    token_service::unauthenticated()
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
