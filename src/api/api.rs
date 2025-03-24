
use std::{path::Path, result};

use actix_web::{body, delete, get, post, web::{self, get, Json}, App, HttpRequest, HttpResponse, HttpServer, Responder};
use anyhow::Ok;
use serde_json::json;
use crate::{helpers::auth::{self, JwtMiddleware}, models::{attend::{self, Attend}, event::{self, Event, Event_Detail}, profile::Profile, request::{LoginQuery, NearbyEventsRequest}}, repo::{database::DataBase, repository::Repository}};
use crate::helpers;
#[post("/profile")]
async fn create_profile(database: web::Data<DataBase>, profile: web::Json<Profile>) -> impl Responder {
    let repo = Repository::<Profile>::new(database.pool.clone());
    // let date_of_birth = profile.date_of_birth.as_ref()
    //     .map(|date| date.format("%Y-%m-%d").to_string())  // If `Some`, format as a string
    //     .unwrap_or("NULL".to_string());  // If `None`, use "NULL"
    
    // let date_joined = profile.date_joined.as_ref()
    //     .map(|date| date.format("%Y-%m-%d").to_string())  // If `Some`, format as a string
    //     .unwrap_or("NULL".to_string());  

    let result = repo
        .create(
            "profile",
            "first_name, last_name, pwd, gender, phone_number, email, date_of_birth, photo_url, emergency_contact, is_active, is_verified, verified_doc_id, about, date_joined, score, lat, lon, comments_id",  
            &format!(
                "'{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}'",
                // profile.profile_id,
                profile.first_name,
                profile.last_name,
                profile.pwd,
                profile.gender,
                profile.phone_number,
                profile.email,
                profile.date_of_birth,
                profile.photo_url.clone().unwrap_or_else(|| "".to_string()),
                profile.emergency_contact,
                profile.is_active,
                profile.is_verified,
                profile.verified_doc_id.unwrap_or(0),
                profile.about.clone().unwrap_or_else(|| "".to_string()),
                profile.date_joined,
                profile.score,
                profile.lat,
                profile.lon,
                profile.comments_id.unwrap_or(0)
            ),
            "profile_id"
        )
        .await;

    match result {
        Result::Ok(profile_id) => {
            HttpResponse::Ok().json(json!({ "profile_id": profile_id }))
        },
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create profile")
        }
    }

}
#[post("/profile/update/{uuid}")]
async fn update_profile(database: web::Data<DataBase>, uuid: web::Path<String>, profile: web::Json<Profile>) -> impl Responder {
    let repo = Repository::<Profile>::new(database.pool.clone());

    // let date_of_birth = match profile.date_of_birth {
    //     Some(date) => format!("'{}'", date.format("%Y-%m-%d")), // Format as 'YYYY-MM-DD'
    //     None => "NULL".to_string(),
    // };

    // let date_joined = match profile.date_joined {
    //     Some(date) => format!("'{}'", date.format("%Y-%m-%d")), // Format as 'YYYY-MM-DD'
    //     None => "NULL".to_string(),
    // };

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
            date_joined = '{}'::TIMESTAMP, 
            score = {}, 
            lat = {}, 
            lon = {}, 
            comments_id = {} 
        WHERE profile_id = '{}'",
            profile.first_name,
            profile.last_name,
            profile.pwd,
            profile.gender,
            profile.phone_number,
            profile.email,
            profile.date_of_birth, 
            profile.photo_url.clone().unwrap_or_else(|| "".to_string()),
            profile.emergency_contact,
            profile.is_active,
            profile.is_verified,
            profile.verified_doc_id.unwrap_or(0),
            profile.about.clone().unwrap_or_else(|| "".to_string()),
            profile.date_joined, 
            profile.score,
            profile.lat,
            profile.lon,
            profile.comments_id.unwrap_or(0),
            uuid
    );

    let result = sqlx::query(&query).execute(&repo.pool).await;

    match result {
        Result::Ok(_) => HttpResponse::Ok().body("Profile updated successfully"),
        Err(err) => {
            eprintln!("Failed to update profile: {}", err);
            HttpResponse::InternalServerError().body("Failed to update profile")
        }
    }
}

#[delete("/profile/{uuid}")]
async fn delete_profile(database: web::Data<DataBase>, uuid: web::Path<i64>) -> impl Responder {
    let repo = Repository::<Profile>::new(database.pool.clone());
    let result = repo.delete("profile","profile_id", uuid.into_inner()).await;

    match result {
        Result::Ok(_) => HttpResponse::Ok().body("Profile successfully deleted."),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete profile")
        }
    }
}

#[get("/profile/{uuid}")]
async fn get_profile_by_id(database: web::Data<DataBase>, id: web::Path<i64>) -> impl Responder {
    let repo = Repository::<Profile>::new(database.pool.clone());
    let result = repo.get_by_id::<Profile>("profile", "profile_id", id.into_inner().try_into().unwrap()).await;

    match result {
        Result::Ok(Some(profile)) => {
            HttpResponse::Ok().json(profile)
        }
        Result::Ok(None) => {
            HttpResponse::NotFound().body("Profile not found")
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch profile data")
        }
    }
}


#[post("/event")]
async fn create_new_event(database: web::Data<DataBase>, event: web::Json<Event_Detail>) -> impl Responder {
    let repo = Repository::<Event_Detail>::new(database.pool.clone());
    // let end_time = event.end_time.as_ref()
    //     .map(|date| date.format("%Y-%m-%d %H:%M").to_string())  // If `Some`, format as a string
    //     .unwrap_or("NULL".to_string());  // If `None`, use "NULL"

    let result = repo.create(
            "events", 
            "name, description, start_time, end_time, 
            latitude, longitude, address, town_name, organizer_id, 
            created_at, updated_at, attendees, max_attendees, ticket_price, category",
           
            &format!("'{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}','{}', '{}', '{}', '{}', '{}'",
                event.name,
                event.description.clone().unwrap_or_else(|| "".to_string()),
                event.start_time,
                event.end_time,
                event.latitude,
                event.longitude,
                event.address.clone().unwrap_or_else(|| "".to_string()),
                event.town_name.clone().unwrap_or_else(|| "".to_string()),
                event.organizer_id.unwrap_or(0),
                event.created_at,
                event.updated_at,
                event.attendees,
                event.max_attendees,
                event.ticket_price.unwrap_or(0.0),
                event.category.clone().unwrap_or_else(|| "".to_string())
            ),
            "event_id"
        ).await;

    match result {
        Result::Ok(_) => {
            HttpResponse::Ok().body("Event created successfully")
        },
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create event")
        }
    }
   
}

#[get("/events")]
async fn get_nearby_events(db: web::Data<DataBase>, req: web::Query<NearbyEventsRequest>) -> impl Responder {
    let radius_in_meter = req.radius * 1000.0;
    let offset = (req.page - 1) * req.limit;

    let repo = Repository::<Event>::new(db.pool.clone());

    let result = repo.get_nearby_events("events", req.latitude, req.longitude, radius_in_meter, req.limit, offset).await;

    match result {
        Result::Ok(events) => {
            HttpResponse::Ok().json(events)
        },
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get nearby events")
        }
    }
}

#[get("/event/{event_id}")]
async fn get_event_by_id(db: web::Data<DataBase>, event_id: web::Path<i64>) -> impl Responder {
    let repo = Repository::<Event_Detail>::new(db.pool.clone());

    let result = repo.get_by_id::<Event_Detail>("events", "event_id", event_id.into_inner().try_into().unwrap()).await;

    match result {
        Result::Ok(Some(event)) => {
            HttpResponse::Ok().json(event)
        },
        Result::Ok(None) => {
            HttpResponse::NotFound().body("Event not found")
        },
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get event detail")
        }
    }
}

#[post("/event/attend")] 
async fn set_event_attend(db: web::Data<DataBase>, attend: web::Json<Attend>) -> impl Responder {
    let repo = Repository::<Attend>::new(db.pool.clone());
    let result = repo.create("attendees", 
        "profile_id, 
        event_id, 
        status, 
        ticket_qr_code, 
        created_at", 
        &format!("'{}', '{}', '{}', '{}', '{}'", 
            attend.profile_id,
            attend.event_id,
            attend.status,
            attend.ticket_qr_code,
            attend.created_at
        ),
        "attendee_id",
    ).await;

    match result {
        Result::Ok(_) => {
            HttpResponse::Ok().body("Attended successfully")
        },
        Result::Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create attend")
        }
    }
}

#[get("/event/{event_id}/attendees")] 
async fn get_event_attendees(db: web::Data<DataBase>, event_id: web::Path<i32>) -> impl Responder {
    let repo = Repository::<Attend>::new(db.pool.clone());

    let result = repo.get_attendees_ids(event_id.into_inner().try_into().unwrap()).await;

    match result {
        Result::Ok(ids) => {
            let repo = Repository::<Profile>::new(db.pool.clone());
            let result = repo.get_profiles_by_ids(&ids).await;

            match result {
                Result::Ok(profiles) => {
                    HttpResponse::Ok().json(profiles)
                },
                Result::Err(e) => {
                    eprintln!("Database error: {:?}", e);
                    HttpResponse::InternalServerError().body("Failed to get event detail")
                }
            }
            
        },
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get event detail")
        }
    }
}


#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}


#[get("/login")]
async fn login(database: web::Data<DataBase>, query: web::Query<LoginQuery>) -> impl Responder {
    println!("{:?}", query);
    match helpers::auth::verify_social_token(&query.provider, &query.access_token).await {
        Result::Ok(id) => {
            
            match helpers::auth::get_google_user_info(&query.provider, &query.access_token, query.lat, query.lon).await {
                Result::Ok(profile) => {
                    let repo = Repository::<Profile>::new(database.pool.clone());
                    match repo.record_exists("profile", "email", &profile.email, "profile_id").await {
                        Result::Ok(Some(profile_id)) => {

                            match repo.get_by_id::<Profile>("profile", "profile_id", profile_id).await {
                       
                                Result::Ok(Some(profile)) => {
                                    match auth::create_jwt(profile_id.into()) {
                                        Result::Ok(token) => HttpResponse::Ok().json(json!({
                                            "access_token": token,
                                            "user_id": profile_id,
                                            "username": profile.first_name,
                                        })),
                                        Result::Err(_) => HttpResponse::Unauthorized().body("Failed to generate token"),
                                    }
                                }
                                Result::Ok(None) => {
                                    HttpResponse::NotFound().body("Profile not found")
                                }
                                Err(e) => {
                                    eprintln!("Database error: {:?}", e);
                                    HttpResponse::InternalServerError().body("Failed to fetch profile data")
                                }
                                
                            }
                        },
                        Result::Ok(None) => {
                            let result = repo
                            .create(
                                "profile",
                                "first_name, last_name, pwd, gender, phone_number, email, date_of_birth, photo_url, emergency_contact, is_active, is_verified, verified_doc_id, about, date_joined, score, lat, lon, comments_id",
                                &format!(
                                    "'{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}'",
                                    profile.first_name,
                                    profile.last_name,
                                    profile.pwd,
                                    profile.gender,
                                    profile.phone_number,
                                    profile.email,
                                    profile.date_of_birth,
                                    profile.photo_url.unwrap_or_else(|| "".to_string()),
                                    profile.emergency_contact,
                                    profile.is_active,
                                    profile.is_verified,
                                    profile.verified_doc_id.unwrap_or(0),
                                    profile.about.unwrap_or_else(|| "".to_string()),
                                    profile.date_joined,
                                    profile.score,
                                    profile.lat,
                                    profile.lon,
                                    profile.comments_id.unwrap_or(0)
                                ),
                                "profile_id"
                            )
                            .await;
            
                            match result {
                                Result::Ok(profile_id) => {
                                    if let Result::Ok(token) = auth::create_jwt(profile_id.into()) {
                                        let response_body = json!({
                                            "access_token": token,
                                            "user_id": profile_id,
                                            "username": profile.first_name,
                                            // "email": user.email,
                                            // "first_name": user.first_name,
                                            // "last_name": user.last_name,
                                        });
                                        HttpResponse::Ok().json(response_body)
                                    } else {
                                        HttpResponse::Unauthorized().body("Invalid credentials")
                                    }  
                                },
                                Err(e) => {
                                    eprintln!("Database error: {:?}", e);
                                    HttpResponse::InternalServerError().body("Failed to create profile")
                                }
                            }
                        },
                        Result::Err(e) => {
                            eprintln!("Database error: {:?}", e);
                            HttpResponse::InternalServerError().body("Database query failed")
                        }
                    }
                    

                    
                    
                },
                Result::Err(e) => HttpResponse::BadRequest().body(format!("Error to get user info: {}", e))
            }

            


            
        }
        Result::Err(e) => HttpResponse::BadRequest().body(format!("Error verifying token: {}", e)),
    }
}


#[get("/verify_token")]
pub async fn verify_jwt(req: HttpRequest) -> HttpResponse {
    match auth::verify_jwt(req).await {
        Result::Ok(user_id) => {
            HttpResponse::Ok().json(json!({ "user_id": user_id}))
        },
        Result::Err(e) => HttpResponse::Unauthorized().body(format!("Error verifying token: {}", e)),
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(ping)
            .service(login)
            .service(verify_jwt)
            .service(
                web::scope("")
                    .wrap(JwtMiddleware::new("thisisnojoke".to_string()))
                    .service(create_profile)
                    .service(update_profile)
                    .service(delete_profile)
                    .service(get_profile_by_id)
                    .service(create_new_event)
                    .service(get_nearby_events)
                    .service(get_event_by_id)
                    .service(set_event_attend)
                    .service(get_event_attendees)

            )
    );
}
