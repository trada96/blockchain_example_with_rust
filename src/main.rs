use actix_web::{web, App, HttpServer};
use std::{env, panic}; 
use std::sync::Mutex;
use uuid::Uuid;

pub mod blockchain;
pub mod api;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Get port from cmd/env, default = 5000
    let port = match args.as_slice(){
        [_, key, value] => {
            if key == "--p" {
                value
            } else {
                panic!("Tham so khong hop le")
            }
        }
        _=> "5000"
    };

    let sharedchain = web::Data::new(Mutex::new(blockchain::Blockchain::new()));
    let node_identifier = web::Data::new(Uuid::new_v4().to_simple().to_string());
    
    println!("ID: {}", Uuid::new_v4().to_simple().to_string());
    
    HttpServer::new(move || {
        App::new()
            .register_data(sharedchain.clone())
            .register_data(node_identifier.clone())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/transaction/new").route(web::post().to(api::new_transaction)))
            .service(web::resource("/mine").route(web::get().to(api::mine)))
            .service(web::resource("/chain").route(web::get().to(api::chain)))
            .service(web::resource("/nodes/register").route(web::post().to(api::register_node)))
            .service(web::resource("/nodes/resolve").route(web::get().to(api::resolve_nodes)))

    })
    .bind(format!("127.0.0.1:{}", port))
    .unwrap()
    .run();

            
}
