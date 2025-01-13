use std::marker::PhantomData;

use sqlx::{query, Error, Pool, Postgres};

pub struct Repository<T> {
    pub pool: Pool<Postgres>,
    pub _marker: std::marker::PhantomData<T>
}

impl <T> Repository<T> {
    pub fn new(pool: Pool<Postgres>) -> Self{
        Repository { pool, _marker: PhantomData }
    }

    pub async fn create(&self, table_name: &str, columns: &str, values: &str) -> Result<(), Error>{
        let query = format!("INSERT INTO {} ({}) VALUES ({})", table_name, columns, values);
        sqlx::query(&query).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update(&self, table_name: &str, id: &str, updates: &str) -> Result<(), Error>{
        let query = format!("UPDATE {} SET {} WHERE uuid = $1", table_name, updates);
        sqlx::query(&query).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn delete(&self, table_name: &str, id: &str) -> Result<(), Error>{
        let query = format!("DELETE FROM {} WHERE uuid = $1", table_name);
        sqlx::query(&query).bind(id).execute(&self.pool).await?;
        Ok(())
    }
    
    pub async fn get_by_id<R>(&self, table_name: &str, id: &str) -> Result<Option<R>, Error>
    where
        R: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin, // Add Send + Unpin
    {
        let query = format!("SELECT * FROM {} WHERE uuid = $1", table_name);

        let result = sqlx::query_as::<_, R>(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

}