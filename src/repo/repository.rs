use std::marker::PhantomData;

use sqlx::{query, Error, Pool, Postgres, Row};

use crate::models::{event::Event, profile::{Profile, ProfileView}};

pub struct Repository<T> {
    pub pool: Pool<Postgres>,
    pub _marker: std::marker::PhantomData<T>
}

impl <T> Repository<T> {
    pub fn new(pool: Pool<Postgres>) -> Self{
        Repository { pool, _marker: PhantomData }
    }

    pub async fn get_attendees_ids(&self, event_id: i32) -> Result<Vec<i32>, Error> {
        let query = format!("SELECT user_id FROM attendees WHERE event_id = {}", event_id);
        
        let result = sqlx::query_scalar::<_, i32>(&query)
            .fetch_all(&self.pool)
            .await?;
        
        Ok(result)
    }

    pub async fn get_profiles_by_ids(&self, profile_ids: &[i32]) -> Result<Vec<ProfileView>, Error> {
        if profile_ids.is_empty() {
            return Ok(vec![]); // Return empty if no IDs
        }
    
        let query = format!(
            "SELECT profile_id, first_name, phone_number, email, date_joined, is_verified, about, photo_url FROM profile WHERE profile_id = ANY($1)"
        );
    
        let result = sqlx::query_as::<_, ProfileView>(&query)
            .bind(profile_ids)
            .fetch_all(&self.pool)
            .await?;
    
        Ok(result)
    }
    

    pub async fn record_exists(&self, table_name: &str, column: &str, value: &str, id_column: &str) -> Result<Option<i32>, Error> {
        let query = format!("SELECT {} FROM {} WHERE {} = $1 LIMIT 1", id_column, table_name, column);
    
        let row: Option<(i32,)> = sqlx::query_as(&query)
            .bind(value) // Prevent SQL injection
            .fetch_optional(&self.pool)
            .await?;
    
        Ok(row.map(|(id,)| id))
    }
    

    pub async fn create(&self, table_name: &str, columns: &str, values: &str, return_val: &str) -> Result<i32, Error>{
        let query = format!("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", table_name, columns, values, return_val);
        let row = sqlx::query(&query)
            .fetch_one(&self.pool)
            .await?;

        let profile_id: i32 = row.try_get(return_val)?;
        Ok(profile_id)
    }

    pub async fn update(&self, table_name: &str, col_id: &str, id: i64, updates: &str) -> Result<(), Error>{
        let query = format!("UPDATE {} SET {} WHERE {} = $1", table_name, updates, col_id);
        sqlx::query(&query).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn delete(&self, table_name: &str, col_id: &str, id: i64) -> Result<(), Error>{
        let query = format!("DELETE FROM {} WHERE {} = $1", table_name, col_id);
        sqlx::query(&query).bind(id).execute(&self.pool).await?;
        Ok(())
    }
    
    pub async fn get_by_id<R>(&self, table_name: &str, col_id: &str, id: i32) -> Result<Option<R>, Error>
    where
        R: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin, // Add Send + Unpin
    {
        let query = format!("SELECT * FROM {} WHERE {} = $1", table_name, col_id);

        let result = sqlx::query_as::<_, R>(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn get_nearby_events(self, table_name: &str, lat: f64, lon: f64, radius: f64, limit: i32, offset: i32) -> Result<Vec<Event>, sqlx::Error> {
        let query = format!(
            r#"
            SELECT event_id, name, start_time, address, town_name, attendees, category
            FROM {}
            WHERE ST_Distance(
                ST_MakePoint(latitude, longitude)::geography,
                ST_MakePoint($1, $2)::geography
            ) <= $3
            ORDER BY ST_Distance(
                ST_MakePoint(latitude, longitude)::geography,
                ST_MakePoint($1, $2)::geography
            )
            LIMIT $4 OFFSET $5
            "#,
            table_name
        );

        let result = sqlx::query_as::<_, Event>(&query)
            .bind(lat)
            .bind(lon)
            .bind(radius)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

            Ok(result)
    }

}