use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Drink {
    pub id: Uuid,
    pub name: String,
    pub brand: String,
    pub caffeine_content: i32,
    pub sugar_content: i32,
    pub price: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDrink {
    pub name: String,
    pub brand: String,
    pub caffeine_content: i32,
    pub sugar_content: i32,
    pub price: f64,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateDrink {
    pub name: Option<String>,
    pub brand: Option<String>,
    pub caffeine_content: Option<i32>,
    pub sugar_content: Option<i32>,
    pub price: Option<f64>,
}
