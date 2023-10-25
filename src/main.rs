mod database;
mod routes;

use crate::database::db::initialize_db_pool;
use crate::routes::*;

use std::env;
use dotenvy::dotenv;
use actix_web::{HttpServer, App, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_pool = initialize_db_pool();

    let host = env::var("SERVER_HOST").expect("SERVER_HOST must be set");
    let port: u16 = env::var("SERVER_PORT").expect("SERVER_PORT must be set")
        .parse::<u16>().unwrap();
    println!("Starting HTTP server on {host}:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(get_tags_for_song)
            .service(add_new_tag)
            .service(delete_tag)
            .service(add_tag_for_song)
            .service(get_tags)
            .service(remove_tag_from_song)
    })
        .bind((host, port))?
        .run()
        .await
}
