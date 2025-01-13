
use actix_web::{body, delete, get, post, web::{self, get}, App, HttpResponse, HttpServer, Responder};
use sqlx::database;
use tokio_tungstenite::tungstenite::protocol::frame::coding::Data;

use crate::{models::profile::Profile, repo::{database::DataBase, repository::Repository}};

#[get("/instruments")]
async fn hello(database: web::Data<DataBase>) -> impl Responder {
    HttpResponse::Ok().body("Hello from scope")
}

#[post("/profile")]
async fn create_profile(database: web::Data<DataBase>, profile: web::Json<Profile>) -> impl Responder {
    let repo = Repository::<Profile>::new(database.pool.clone());
    let date_of_birth = profile.date_of_birth.as_ref()
        .map(|date| date.format("%Y-%m-%d").to_string())  // If `Some`, format as a string
        .unwrap_or("NULL".to_string());  // If `None`, use "NULL"
    
    let date_joined = profile.date_joined.as_ref()
        .map(|date| date.format("%Y-%m-%d").to_string())  // If `Some`, format as a string
        .unwrap_or("NULL".to_string());  

    let result = repo
        .create(
            "profile",
            "uuid, first_name, last_name, pwd, gender, phone_number, email, date_of_birth, photo_url, emergency_contact, is_active, is_verified, verified_doc_id, about, date_joined, score, lat, lon, comments_id",
            &format!(
                "'{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}'",
                profile.uuid,
                profile.first_name,
                profile.last_name,
                profile.pwd,
                profile.gender,
                profile.phone_number,
                profile.email,
                date_of_birth,
                profile.photo_url.clone().unwrap_or_else(|| "".to_string()),
                profile.emergency_contact,
                profile.is_active,
                profile.is_verified,
                profile.verified_doc_id.unwrap_or(0),
                profile.about.clone().unwrap_or_else(|| "".to_string()),
                date_joined,
                profile.score,
                profile.lat.unwrap_or(0.0),
                profile.lon.unwrap_or(0.0),
                profile.comments_id.unwrap_or(0)
            ),
        )
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Profile created successfully"),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create profile")
        }
    }

}
#[post("/profile/update/{uuid}")]
async fn update_profile(database: web::Data<DataBase>, uuid: web::Path<String>, profile: web::Json<Profile>) -> impl Responder {
    let repo = Repository::<Profile>::new(database.pool.clone());

    let date_of_birth = match profile.date_of_birth {
        Some(date) => format!("'{}'", date.format("%Y-%m-%d")), // Format as 'YYYY-MM-DD'
        None => "NULL".to_string(),
    };

    let date_joined = match profile.date_joined {
        Some(date) => format!("'{}'", date.format("%Y-%m-%d")), // Format as 'YYYY-MM-DD'
        None => "NULL".to_string(),
    };

    let query = format!(
        "UPDATE profile SET 
            first_name = '{}', 
            last_name = '{}', 
            pwd = '{}', 
            gender = {}, 
            phone_number = '{}', 
            email = '{}', 
            date_of_birth = {}, 
            photo_url = '{}', 
            emergency_contact = {}, 
            is_active = {}, 
            is_verified = {}, 
            verified_doc_id = {}, 
            about = '{}', 
            date_joined = {}, 
            score = {}, 
            lat = {}, 
            lon = {}, 
            comments_id = {} 
        WHERE uuid = '{}'",
        profile.first_name,
        profile.last_name,
        profile.pwd,
        profile.gender,
        profile.phone_number,
        profile.email,
        date_of_birth, 
        profile.photo_url.clone().unwrap_or_else(|| "".to_string()),
        profile.emergency_contact,
        profile.is_active,
        profile.is_verified,
        profile.verified_doc_id.unwrap_or(0),
        profile.about.clone().unwrap_or_else(|| "".to_string()),
        date_joined, 
        profile.score,
        profile.lat.unwrap_or(0.0),
        profile.lon.unwrap_or(0.0),
        profile.comments_id.unwrap_or(0),
        uuid
    );

    let result = sqlx::query(&query).execute(&repo.pool).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Profile updated successfully"),
        Err(err) => {
            eprintln!("Failed to update profile: {}", err);
            HttpResponse::InternalServerError().body("Failed to update profile")
        }
    }
}

#[delete("/profile/{uuid}")]
async fn delete_profile(database: web::Data<DataBase>, uuid: web::Path<String>) -> impl Responder {
    let repo = Repository::<Profile>::new(database.pool.clone());
    let result = repo.delete("profile", &uuid).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User successfully deleted."),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete user")
        }
    }
}

#[get("/profile/{uuid}")]
async fn get_profile_by_id(database: web::Data<DataBase>, id: web::Path<String>) -> impl Responder {
    let repo = Repository::<Profile>::new(database.pool.clone());
    let result = repo.get_by_id::<Profile>("profile", &id).await;

    match result {
        Ok(Some(profile)) => {
            HttpResponse::Ok().json(profile)
        }
        Ok(None) => {
            HttpResponse::NotFound().body("Profile not found")
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch profile data")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .service(hello)
        .service(create_profile)
        .service(update_profile)
        .service(delete_profile)
        .service(get_profile_by_id)
    );
}