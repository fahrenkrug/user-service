use tonic::{Request, Response, Status};
use crate::{Pool, PooledConnection};
use super::super::authentication::{LoginRequest, LoginResponse};
use super::super::token_service::create_token;
use diesel::prelude::*;
use diesel::sql_types::Text;
use uuid::Uuid;
use crate::authentication_service::email_identity_service::EmailIdentityService;

type ResponseResult = Result<Response<LoginResponse>, Status>;

pub fn respond(pool: &Pool, request: Request<LoginRequest>) -> ResponseResult {
    println!("Got login request: {:?}", request);
    let request_data = request.into_inner();
    let connection = pool.get().unwrap();
    match_user_with_hash(&connection, &request_data.email, &request_data.password)
}

fn match_user_with_hash(connection: &PooledConnection, user_email: &str, password: &str) -> ResponseResult {
    use crate::schema::email_identities::dsl::*;
    sql_function!(fn lower(x: Text) -> Text);
    match email_identities.filter(lower(email).eq(user_email)).filter(created_at.eq(updated_at)).order(created_at.desc()).select((hash, user_id)).first::<(String, Uuid)>(connection) {
        Ok((password_hash, user_uuid)) => match_password_with_hash(password, password_hash, &user_uuid),
        Err(e) => {
            println!("error in login: {}", e);
            err()
        }
    }
}

fn match_password_with_hash(password: &str, hash: String, user_id: &Uuid) -> ResponseResult {
    match EmailIdentityService::matches(password, hash) {
        true => {
            let token = create_token(user_id);
            let response = LoginResponse {
                token: format!("Hello {}!", token),
            };
            Ok(Response::new(response))
        },
        false => err()
    }
}

fn err() -> ResponseResult {
    Err(Status::unauthenticated("Credentials don't match or user does not exist."))
}