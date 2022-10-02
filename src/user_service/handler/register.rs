use super::super::email_identity_service::EmailIdentityService;
use super::super::user::{RegisterRequest, RegisterResponse};
use crate::diesel::RunQueryDsl;
use crate::models::users::{NewUser, User};
use crate::Pool;
use regex::Regex;
use tonic::{Request, Response, Status};

pub async fn respond(
    pool: &Pool,
    request: Request<RegisterRequest>,
) -> Result<Response<RegisterResponse>, Status> {
    println!("Got register request: {:?}", request);
    let request_data = request.into_inner();
    let password_service = EmailIdentityService::new();
    let does_exist = password_service
        .does_email_exist(pool, &request_data.email)
        .await;
    if does_exist {
        return Err(Status::already_exists(
            "A user with this email already exists",
        ));
    }
    let new_user = new_user_from_request(&request_data);
    let user: User = store_user(pool, &new_user);
    let pool = pool.clone();
    tokio::spawn(async move {
        let password_service = EmailIdentityService::new();
        futures::executor::block_on(password_service.store_email_identity(
            &pool,
            &request_data.password,
            &user.id,
            &request_data.email,
        ))
    });
    println!("Returning register response");
    response_from_user(&user)
}

fn new_user_from_request(request: &RegisterRequest) -> NewUser {
    NewUser {
        id: None,
        first_name: &request.first_name,
        last_name: &request.last_name,
        created_at: None,
        updated_at: None,
    }
}

fn response_from_user(user: &User) -> Result<Response<RegisterResponse>, Status> {
    let response = RegisterResponse {
        token: format!("Hello {}!", user.first_name),
        user_id: format!("uuuid-test {:?}", user.id),
    };
    Ok(Response::new(response))
}

fn store_user(pool: &Pool, new_user: &NewUser) -> User {
    use crate::schema::users;
    let connection = pool.get().unwrap();
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(&connection)
        .expect("Error saving new user.")
}

// fn verify_request_data(data: &RegisterRequest) -> bool {}

fn verify_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-.][a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    email_regex.is_match(email)
}
