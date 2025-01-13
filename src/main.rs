use actix_web::{middleware, web, App, HttpServer };
use TownLink_server::{api::api, repo::database::{self, DataBase}};
use sqlx::{Pool, Postgres};
use std::env;

mod repo;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Hello, world!");

    let database = DataBase::new().await;

    database.initialize().await;

    let app_data = web::Data::new(database);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            // .wrap(middleware::Compress::default())
            .configure(api::config)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


