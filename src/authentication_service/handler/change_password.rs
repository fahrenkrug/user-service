use tonic::{Request, Response, Status};
use super::super::{ChangePasswordRequest, ChangePasswordResponse};

pub fn respond(request: Request<ChangePasswordRequest>,) -> Result<Response<ChangePasswordResponse>, Status> {
    println!("Got change password request: {:?}", request);
    let response = ChangePasswordResponse {
        token: format!("Hello {}!", request.into_inner().new_password),
    };

    Ok(Response::new(response))
}