mod errors;
mod ws_sess_manager;
mod ws_actor;

use actix::prelude::*;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Error};
use actix_web_actors::ws;
use crate::errors::ConnectionError;
use crate::ws_actor::WebSocket;
use crate::ws_sess_manager::WsSessionManager;



/// Setting up WebSocket endpoint on active http server
async fn ws_index( req: HttpRequest, stream: web::Payload, srv: web::Data<Addr<WsSessionManager>> )
    -> Result<HttpResponse, Error> {

    let ws_actor = WebSocket {
        manager: srv.get_ref().clone(),
    };

    ws::start(ws_actor, &req, stream)
}


/// Setting up server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = WsSessionManager::new().start();
    let address = "0.0.0.0:8080";

    let server = HttpServer::new(move || {
        App::new()
            .route("/ws/", web::get().to(ws_index))
            .service(actix_files::Files::new("/", "public").index_file("index.html"))
            .data(manager.clone())
    }).workers(1).bind(address);

    match server {
        Ok(server) => {
            println!("Server listening on: {}", address);
            server.run().await
        }
        Err(err) => {
            panic!("{}", ConnectionError::CreateServerError(err.to_string()))
        }
    }
}
