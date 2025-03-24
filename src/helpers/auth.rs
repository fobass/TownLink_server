use actix_web::body::BoxBody;
use actix_web::dev::ServiceResponse;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use chrono::Utc;
use reqwest::Client;
use serde_json::{json, Value};

use crate::models::profile::Profile;
use crate::repo::database::DataBase;


use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
// use std::task::{Context, Poll};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::models::token::Claims;
use jsonwebtoken::{decode, Validation, DecodingKey};
// use serde::{Serialize, Deserialize};
use actix_web::error::ErrorUnauthorized;
use actix_web::HttpResponse;

use actix_web::Result;
use actix_web::{web, Error, dev::ServiceRequest};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::rc::Rc;
use std::task::{Context, Poll};
use actix_web::dev::{Service, Transform};

// Middleware Struct
pub struct JwtMiddleware {
    secret: String,
}

impl JwtMiddleware {
    pub fn new(secret: String) -> Self {
        JwtMiddleware { secret }
    }
}

impl<S> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService {
            service: Rc::new(service),
            secret: self.secret.clone(),
        })
    }
}


// Middleware Service
pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
    secret: String,
}

impl<S> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    
    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }
    // forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret = self.secret.clone();
        let svc = self.service.clone();

        Box::pin(async move {
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|header| header.to_str().ok());

            if let Some(auth) = auth_header {
                if let Some(token) = auth.strip_prefix("Bearer ") {
                    let decoding_key = DecodingKey::from_secret(secret.as_ref());

                    match decode::<Claims>(token, &decoding_key, &Validation::default()) {
                        Ok(decoded) => {
                            req.extensions_mut().insert(decoded.claims.user_id);
                            return svc.call(req).await;
                        }
                        Err(_) => {
                            return Ok(req.into_response(
                                HttpResponse::Unauthorized().finish().map_into_boxed_body(),
                            ));
                        }
                    }
                }
            }

            Ok(req.into_response(
                HttpResponse::Unauthorized().finish().map_into_boxed_body(),
            ))
        })
    }
}

pub fn create_jwt(user_id: i64) -> Result<String, Box<dyn std::error::Error>> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() + 30 * 24 * 60 * 60; // 1 hour expiration
    
    let claims = Claims {
        user_id,
        exp: expiration as usize,
    };
    
    let secret = "thisisnojoke"; // Store this securely (in environment variables or config)
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    
    Ok(token)
}

pub async fn verify_jwt(req: HttpRequest) -> Result<i64, Box<dyn std::error::Error>> {
    let token = extract_token(&req)?;
    
    let secret = "thisisnojoke"; // Same key used for encoding
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256)
    ).map_err(|_| ErrorUnauthorized("Invalid token"))?;
    
    let user_id = decoded.claims.user_id;

    // Fetch the user from the database based on the user_id extracted from the token
    
    Ok(user_id)
}

// Helper function to extract the JWT from the Authorization header
fn extract_token(req: &HttpRequest) -> Result<String, actix_web::Error> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        let auth_header = auth_header.to_str().map_err(|_| ErrorUnauthorized("Invalid Authorization header"))?;
        if auth_header.starts_with("Bearer ") {
            Ok(auth_header[7..].to_string())
        } else {
            Err(ErrorUnauthorized("Invalid token format"))
        }
    } else {
        Err(ErrorUnauthorized("Missing Authorization header"))
    }
}

fn clean_profile_picture_url(url: &str) -> String {
    url.split("\u{003d}").next().unwrap_or(url).to_string()
}

pub(crate) async fn get_google_user_info(provider: &str, access_token: &str, lat: f64, lon: f64) -> Result<Profile, Box<dyn std::error::Error>> {
    let client = Client::new();

    let url = match provider {
        "google" => format!("https://www.googleapis.com/oauth2/v3/userinfo?access_token={}", access_token),
        "facebook" => format!("https://graph.facebook.com/me?access_token={}&fields=id,name,email", access_token),
        _ => return Err("Unsupported provider".into()),  // Convert &str into a Box<dyn Error>
    };

    let response = client.get(&url).send().await?.json::<Value>().await?;

    let profile = match provider {
        "google" => {
            Profile {
                profile_id: 0,
                first_name: response["given_name"].as_str().unwrap_or("").to_string(),
                last_name: response["family_name"].as_str().unwrap_or("").to_string(),
                pwd: "".to_string(),
                gender: -1,
                phone_number: "".to_string(),
                email: response["email"].as_str().unwrap_or("").to_string(),
                date_of_birth: Utc::now().naive_utc(), //TODO,
                photo_url: Some(clean_profile_picture_url(&response["picture"].as_str().unwrap_or("").to_string())),
                emergency_contact: -1,
                is_active: true,
                is_verified: false,
                verified_doc_id: Some(-1),
                about: Some("".to_string()),
                date_joined: Utc::now().naive_utc(),
                score: 0,
                lat: lat,
                lon: lon,
                comments_id: Some(-1),
            }
        },
        "facebook" => {
            Profile {
                profile_id: 0,
                first_name: response["first_name"].as_str().unwrap_or("").to_string(),
                last_name: response["last_name"].as_str().unwrap_or("").to_string(),
                pwd: "".to_string(),
                gender: -1,
                phone_number: "".to_string(),
                email: response["email"].as_str().unwrap_or("").to_string(),
                date_of_birth: Utc::now().naive_utc(), //TODO
                photo_url: response["imageURL"].as_str().map(|s| s.to_string()),
                emergency_contact: -1,
                is_active: true,
                is_verified: false,
                verified_doc_id: Some(-1),
                about: Some("".to_string()),
                date_joined: Utc::now().naive_utc(),
                score: 0,
                lat: lat,
                lon: lon,
                comments_id: Some(-1),
            }
        },
        _ => return Err("Featch user info: The data couldn’t be read because it isn’t in the correct format".into())
    };


    Ok(profile)
}


pub(crate) async fn verify_social_token(provider: &str, access_token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = match provider {
        "google" => format!("https://www.googleapis.com/oauth2/v3/tokeninfo?access_token={}", access_token),
        "facebook" => format!("https://graph.facebook.com/me?access_token={}&fields=id,name,email", access_token),
        _ => return Err("Unsupported provider".into()),  // Convert &str into a Box<dyn Error>
    };

    let response = client.get(&url).send().await?.json::<Value>().await?;
    // println!("URL {}", url);
    let id = match provider {
        "google" => {
            response["sub"].as_str().unwrap_or("").to_string()
        },
        "facebook" => {
            response["sub"].as_str().unwrap_or("").to_string()
        },
        _ => return Err("Featch user info: The data couldn’t be read because it isn’t in the correct format".into())
    };


    Ok(id)
}


