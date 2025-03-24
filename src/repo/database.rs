use std::{fmt::format, result};
use sqlx::{Pool, Postgres, FromRow, Error};
use profile::Profile;
use std::env;
use crate::models::profile;

#[derive(Clone)]
pub struct DataBase {
    pub pool: Pool<Postgres>,
    pub connection_str: String
}

impl DataBase {
  
    pub async fn new() -> Self {

        dotenvy::dotenv().ok();
        let db_host = env::var("DATABASE_HOST").expect("DATABASE HOST must be set");
        let db_user = env::var("DATABASE_USER").expect("DARABASE_USER must be set");
        let db_pwd = env::var("DATABASE_PWD").expect("DATABASE_PWD must be set");
        let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    
        let db_connection_string = format!(
            "postgres://{}:{}@{}",
            db_user, db_pwd, db_host
        );
    
        let pool = Pool::<Postgres>::connect(&db_connection_string)
            .await
            .unwrap_or_else(|e| panic!("Failed to connect to the database: {:?}", e));

        // let pool_result = Pool::<Postgres>::connect(&db_connection_string).await;

        // match pool_result {
        //     Ok(pool) => {
        //         println!("Successfully connected to the database");
        //         pool
        //     }
        //     Err(e) => {
        //         eprintln!("Database connection error: {:?}", e);
        //         return Err(e.into()); // Or handle the error appropriately
        //     }
        // };


        let create_db = format!(r#"CREATE DATABASE "{}""#, db_name);
        let result = sqlx::query(&create_db).execute(&pool).await;
        match result {
            Ok(_) => println!("Database '{}' created successfully", db_name),
            Err(e) => {
                if let Some(pg_error) = e.as_database_error() {
                    if pg_error.message().contains("already exists") {
                        println!("Database '{}' already exists", db_name);
                    } else {
                        panic!("Failed to create database '{}': {}", db_name, e);
                    }
                }
            }
        }

        let conn_str = format!("{}/{}", db_connection_string, db_name);

        let db_pool = Pool::<Postgres>::connect(&conn_str)
            .await
            .expect("Failed to connect to the database");

        DataBase { pool: db_pool, connection_str: conn_str }
    }

    pub async fn initialize(&self) {
        // self.create_user_table().await;
        self.create_profile_table().await;
        self.create_event_table().await;
        self.create_attendees_table().await;

        if let Some(db_name) = self.connection_str.split("/").last().map(|a| a.to_string()) {
            println!("Tables initialized successfully in database '{}'", db_name);
        } else {
            println!("No database name found.");
        }
        
    }

    // async fn create_user_table(&self) {
    //     let query = r#"
    //     CREATE TABLE IF NOT EXISTS users (
    //             user_id SERIAL PRIMARY KEY,
    //             profile_id INT,
    //             provider VARCHAR(10), -- "google" or "facebook"
    //             provider_id VARCHAR(255) UNIQUE, -- Google/Facebook user ID
    //             created_at TIMESTAMP DEFAULT now(),
    //             last_login TIMESTAMP DEFAULT now()
    //         );
    //     "#;

    //     if let Err(e) = sqlx::query(query).execute(&self.pool).await {
    //         println!("Error creating table: {}", e);
    //         return;
    //     }
    // }

    async fn create_profile_table(&self){
        let query = r#" 
            CREATE TABLE IF NOT EXISTS profile (
                profile_id SERIAL PRIMARY KEY,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                pwd TEXT NOT NULL,
                gender SMALLINT NOT NULL,
                phone_number TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                date_of_birth TIMESTAMP,
                photo_url TEXT,
                emergency_contact SMALLINT NOT NULL,
                is_active BOOLEAN NOT NULL DEFAULT TRUE,
                is_verified BOOLEAN NOT NULL DEFAULT FALSE,
                verified_doc_id SMALLINT,
                about TEXT,
                date_joined TIMESTAMP DEFAULT NOW(),
                score BIGINT NOT NULL DEFAULT 0,
                lat DOUBLE PRECISION,
                lon DOUBLE PRECISION,
                comments_id INTEGER,
                provider VARCHAR(10), -- "google" or "facebook"
                provider_id VARCHAR(255) UNIQUE, -- Google/Facebook user ID
                last_login TIMESTAMP DEFAULT now()
            );
        "#;


        if let Err(e) = sqlx::query(query).execute(&self.pool).await {
            println!("Error creating table: {}", e);
            return;
        }
    }

    async fn create_event_table(&self){
        let query = r#" 
            CREATE TABLE IF NOT EXISTS events (
                event_id SERIAL PRIMARY KEY ,             -- Unique ID for each event
                name VARCHAR(255) NOT NULL,               -- Name of the event
                description TEXT,                         -- Description of the event
                start_time TIMESTAMP NOT NULL,            -- Start time of the event
                end_time TIMESTAMP,                       -- End time of the event (optional)
                latitude DOUBLE PRECISION NOT NULL,       -- Latitude of the event location
                longitude DOUBLE PRECISION NOT NULL,      -- Longitude of the event location
                address VARCHAR(255),                     -- Address of the event (e.g., "123 Main St")
                town_name VARCHAR(100),                   -- Town or city name (e.g., "New York")
                organizer_id INT,                         -- ID of the event organizer (foreign key)
                created_at TIMESTAMP DEFAULT NOW(),       -- Timestamp when the event was created
                updated_at TIMESTAMP DEFAULT NOW(),        -- Timestamp when the event was last updated
                attendees INT,
                max_attendees INT,
                ticket_price DOUBLE PRECISION NOT NULL, 
                category VARCHAR(100)
            );
        "#;


        if let Err(e) = sqlx::query(query).execute(&self.pool).await {
            println!("Error creating table: {}", e);
            return;
        }
    }

    async fn create_attendees_table(&self){
        let q = r#"
            CREATE TABLE IF NOT EXISTS attendees (
                attendee_id SERIAL PRIMARY KEY, 
                profile_id INT NOT NULL,
                event_id INT NOT NULL,
                status VARCHAR(50), -- e.g., "attending", "interested", "cancelled"
                ticket_qr_code VARCHAR(512), -- URL to the QR code image
                created_at TIMESTAMP DEFAULT NOW()
            )
        "#;

        if let Err(e) = sqlx::query(q).execute(&self.pool).await {
            println!("Error creating table {} ", e);
            return;
        }
    }
} 