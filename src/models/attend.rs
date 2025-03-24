
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres, Pool};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct Attend {
    pub attendee_id: i64,
    pub profile_id: i64,
    pub event_id: i64,
    pub status: String,  // -- e.g., "attending", "interested", "cancelled"
    pub ticket_qr_code: String, //-- Binary data for the QR code
    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    pub created_at: NaiveDateTime
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