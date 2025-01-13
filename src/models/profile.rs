use chrono::{Date, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres, Pool};

use crate::repo::repository::Repository;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct Profile{
    pub uuid: String,
    pub first_name: String,
    pub last_name: String,
    pub pwd: String,
    pub gender: i16,
    pub phone_number: String,
    pub email: String,
    pub date_of_birth: Option<NaiveDate>, // Adjust type if you're using chrono's `NaiveDate`
    pub photo_url: Option<String>,
    pub emergency_contact: i16,
    pub is_active: bool,
    pub is_verified: bool,
    pub verified_doc_id: Option<i16>, // Changed to Option<i16>
    pub about: Option<String>,
    pub date_joined: Option<NaiveDate>, // Optional if you want to default in DB
    pub score: i64,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub comments_id: Option<i32>,


    // pub uuid: String,
    // pub first_name: String, 
    // pub last_name: String,
    // pub pwd: String,
    // pub gender: i16,
    // pub phone_number: String,
    // pub email: String,
    // // #[sqlx(skip)]
    // pub date_of_birth: Option<NaiveDate>,
    // pub photo_url: String,
    // pub emergency_contact: i16,
    // pub is_active: bool,
    // pub is_verified: bool,
    // pub verified_doc_id: i16,
    // pub about: String,
    // // #[sqlx(skip)]
    // pub date_joined: Option<NaiveDate>,
    // pub score: i64,
    // pub lat: f64,
    // pub lon: f64,
    // pub comments_id: i64,
}

impl Profile {
    pub async fn create_repo(pool: Pool<Postgres>) -> Repository<Profile> {
        Repository::new(pool)
    }


}

// impl FromRow {
    
// }

