use std::{net::TcpListener, result::Result};
use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};


async fn health_check(_req: HttpRequest)-> HttpResponse{
    HttpResponse::Ok().finish()
}

pub fn run(listener:TcpListener) -> Result<Server, std::io::Error>{
    let server = HttpServer::new(||{
        App::new()
            .route("/health_check", web::get().to(health_check))
        })
        .listen(listener)?
        .run();
    Ok(server)
}