use tonic::{Request, Response, Status};
use super::super::{ForgotPasswordRequest, ForgotPasswordResponse};

pub fn respond(request: Request<ForgotPasswordRequest>) -> Result<Response<ForgotPasswordResponse>, Status> {
    println!("Got forgot_password request: {:?}", request);
    let response = ForgotPasswordResponse {
        success: true
    };
    Ok(Response::new(response))
}