use actix_web::web::{Data};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
struct Poi {
    id: Option<i32>,
    name: Option<String>,
    category_id: Option<i32>,
    address: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[get("/pois")]
async fn get_poi(pool: Data<PgPool>) -> impl Responder {
    let pois = sqlx::query_as!(
        Poi,
        "
            SELECT *
            FROM pois
        "
    )
    .fetch_all(&**pool) // -> Vec<Poi>
    .await;

    match pois {
        Ok(pois) => HttpResponse::Ok().json(pois),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[derive(Deserialize)]
struct PoiRadiusQuery {
    latitude: f64,
    longitude: f64,
    radius: f64,
}

#[get("/pois-by-radius")]
async fn get_poi_by_radius(
    pool: Data<PgPool>,
    query: web::Query<PoiRadiusQuery>,
) -> impl Responder {
    let pois = sqlx::query_as!(
        Poi,
        "
            SELECT *
            FROM pois
            WHERE ST_DWithin(ST_MakePoint($1, $2)::geography, ST_MakePoint(pois.latitude, pois.longitude)::geography, $3)
        ",
        query.latitude,
        query.longitude,
        query.radius
    )
        .fetch_all(&**pool) // -> Vec<Poi>
        .await;

    match pois {
        Ok(pois) => HttpResponse::Ok().json(pois),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/pois")]
async fn create_poi(pool: Data<PgPool>, json: web::Json<Poi>) -> impl Responder {
    let poi = sqlx::query_as!(
        Poi,
        "
            INSERT INTO pois (name, category_id, address, latitude, longitude)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
        ",
        json.name,
        json.category_id,
        json.address,
        json.latitude,
        json.longitude
    )
    .fetch_one(&**pool) // -> Poi
    .await;

    match poi {
        Ok(poi) => HttpResponse::Ok().json(poi),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to execute query: {:?}", e))
        }
    }
}
