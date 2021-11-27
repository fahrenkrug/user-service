use tonic::{Request, Response, Status};
use super::super::{UserRequest, UserResponse};

pub fn respond(request: Request<UserRequest>) -> Result<Response<UserResponse>, Status> {
    println!("metadata: {:?}", request.metadata());
    let response = UserResponse{};
    Ok(Response::new(response))
}