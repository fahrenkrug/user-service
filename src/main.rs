use actix_protobuf::*;
use actix_web::*;
use prost::Message as Message;

// Include the `items` module, which is generated from items.proto.
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/user.items.rs"));
}

/*
#[derive(Debug)]
struct RegisterDto {
    #[prost(string, tag="email")]
    email: String,

    #[prost(string, tag="password")]
    password: String,
}
 */

async fn register(message: ProtoBuf<items::Register>) -> Result<HttpResponse> {
    println!("Registering: {:?}", message);
    HttpResponse::Ok().protobuf(message.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/register").route(web::post().to(register)))
    })
        .bind("127.0.0.1:8080")?.shutdown_timeout(1).run().await
}
