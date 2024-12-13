use sqlx::{Pool, Postgres, migrate::MigrateDatabase};
use std::env;

pub async fn get_db_pool() -> Pool<Postgres> {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");

    // Ensure that database exists (optional - only if you want to create the DB programmatically)
    if !Postgres::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        println!("Database does not exist. Creating...");
        Postgres::create_database(&database_url).await.unwrap();
    }

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    pool
}
