use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

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

#[derive(serde::Serialize, sqlx::FromRow, Debug)]
pub struct BrandSummary {
    pub brand: String,
    pub drink_count: i64,
    pub avg_caffeine: f64,
    pub avg_sugar: f64,
    pub avg_price: f64,
}

#[derive(serde::Deserialize)]
pub struct DrinkSearchParams {
    pub brand: Option<String>,
    pub min_caffeine: Option<i32>,
    pub max_caffeine: Option<i32>,
    pub min_sugar: Option<i32>,
    pub max_sugar: Option<i32>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
}
