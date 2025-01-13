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
            .expect("Failed to connect to the database");

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
        let query = r#" 
            CREATE TABLE IF NOT EXISTS profile (
                uuid TEXT PRIMARY KEY,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                pwd TEXT NOT NULL,
                gender SMALLINT NOT NULL,
                phone_number TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                date_of_birth DATE NOT NULL,
                photo_url TEXT,
                emergency_contact SMALLINT NOT NULL,
                is_active BOOLEAN NOT NULL DEFAULT TRUE,
                is_verified BOOLEAN NOT NULL DEFAULT FALSE,
                verified_doc_id SMALLINT,
                about TEXT,
                date_joined DATE NOT NULL DEFAULT CURRENT_DATE,
                score BIGINT NOT NULL DEFAULT 0,
                lat DOUBLE PRECISION,
                lon DOUBLE PRECISION,
                comments_id INTEGER
            );
        "#;


        if let Err(e) = sqlx::query(query).execute(&self.pool).await {
            println!("Error creating table: {}", e);
            return;
        }

        // sqlx::query(query)
        //     .execute(&self.pool)
        //     .await
        //     .expect("Failed to create table");

        if let Some(db_name) = self.connection_str.split("/").last().map(|a| a.to_string()) {
            println!("Tables initialized successfully in database '{}'", db_name);
        } else {
            println!("No database name found.");
        }
        
    }

    // pub async add_new_profile()
    // pub async fn get_profile_by_uuid(&self, uuid: &str) -> Result<Option<Profile>, Error> {
    //     let query = r#"
    //         SELECT uuid, first_name, last_name, pwd, gender, phone_number, email, date_of_birth,
    //                photo_url, emergency_contact, is_active, is_verified, verified_doc_id, about,
    //                date_joined, score, lat, lon, comments_id
    //         FROM profiles
    //         WHERE uuid = $1
    //     "#;
    
    //     let profile = sqlx::query_as::<_, Profile>(query)
    //         .bind(uuid)
    //         .fetch_optional(&self.pool)
    //         .await?;
    
    //     Ok(profile)
    // }

} 