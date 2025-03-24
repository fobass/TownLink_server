use actix_web::{middleware, web, App, HttpServer };
use TownLink_server::{api::api, repo::database::{self, DataBase}};
use local_ip_address::local_ip;
use actix_cors::Cors;
mod repo;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let addr: String;
    let mut port = "7777";
    match local_ip() {
        Ok(ip) => {
            let ip_str = ip.to_string();
            addr = ip_str.clone();

            println!("Local Wi-Fi IP: {}", ip)
        },
        Err(_e) => {
            addr = "192.168.0.101".to_string();
            port = "7777";
        },
    }
    println!("TownLink server running at http://{}", addr.clone());
    let database = DataBase::new().await;

    database.initialize().await;
    
    let app_data = web::Data::new(database);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::Compress::default())
            .configure(api::config)
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )

    })
    .bind((addr.clone(), 8080))?
    .run()
    .await
}


