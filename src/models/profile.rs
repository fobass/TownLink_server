use chrono::{Date, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres, Pool};

use crate::repo::repository::Repository;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct Profile{
    pub profile_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub pwd: String,
    pub gender: i16,
    pub phone_number: String,
    pub email: String,
    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    pub date_of_birth: NaiveDateTime, 
    pub photo_url: Option<String>,
    pub emergency_contact: i16,
    pub is_active: bool,
    pub is_verified: bool,
    pub verified_doc_id: Option<i16>, 
    pub about: Option<String>,
    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    pub date_joined: NaiveDateTime,
    pub score: i64,
    pub lat: f64,
    pub lon: f64,
    pub comments_id: Option<i32>,

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct ProfileView {
    pub profile_id: i32,
    pub first_name: String,
    pub phone_number: String,
    pub email: String,
    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    pub date_joined: NaiveDateTime,
    pub is_verified: bool,
    pub about: Option<String>,
    pub photo_url: String,
}

// Helper functions to convert NaiveDateTime <-> UNIX timestamp
fn serialize_naive_datetime<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let timestamp = dt.timestamp(); // Convert NaiveDateTime to UNIX timestamp
    serializer.serialize_i64(timestamp)
}

fn deserialize_naive_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp = i64::deserialize(deserializer)?; // Read UNIX timestamp
    Ok(NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap()) // Convert back to NaiveDateTime
}

impl Profile {
    pub async fn create_repo(pool: Pool<Postgres>) -> Repository<Profile> {
        Repository::new(pool)
    }

}

