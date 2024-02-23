use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct PoiCategory {
    pub id: i32,
    pub name: Option<String>
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct PoiAddress {
    pub id: i32,
    pub street: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub housenumber: Option<String>,
    pub postcode: Option<String>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Poi {
    pub id: i32,
    pub name: Option<String>,
    pub category_id: Option<i32>,
    pub address_id: Option<i32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub slug: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}