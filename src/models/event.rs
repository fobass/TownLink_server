use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres, Pool};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct Event_Detail {
    pub event_id: i32,
    pub name: String,
    pub description: Option<String>,
    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    pub start_time: NaiveDateTime,
    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    pub end_time: NaiveDateTime,
    pub latitude: f64,
    pub longitude: f64,
    pub address: Option<String>,
    pub town_name: Option<String>,
    pub organizer_id: Option<i32>,
    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    pub updated_at: NaiveDateTime,
    pub attendees: i32,
    pub max_attendees: i32,
    pub ticket_price: Option<f64>,
    pub category: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct Event {
    pub event_id: i32,
    pub name: String,
    // pub description: Option<String>,
    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    pub start_time: NaiveDateTime,
    // pub end_time: Option<NaiveDateTime>,
    // pub latitude: f64,
    // pub longitude: f64,
    pub address: Option<String>,
    pub town_name: Option<String>,
    // pub organizer_id: Option<i32>,
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime,
    pub attendees: i32,
    // pub max_attendees: i32,
    // pub ticket_price: Option<f64>,
    pub category: Option<String>
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