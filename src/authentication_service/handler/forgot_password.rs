use tonic::{Request, Response, Status};
use super::super::{ForgotPasswordRequest, ForgotPasswordResponse};

pub fn respond(request: Request<ForgotPasswordRequest>) -> Result<Response<ForgotPasswordResponse>, Status> {
    println!("Got forgot_password request: {:?}", request);
    todo!("Come back here once we have a notification service ready");
    Ok(Response::new(ForgotPasswordResponse {}))
}