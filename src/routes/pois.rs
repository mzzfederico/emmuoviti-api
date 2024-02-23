use actix_web::web::{Data, Query};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize};
use sqlx::PgPool;
use web::Json;
use crate::models::Poi;

#[derive(Deserialize)]
struct Pagination {
    page: Option<i64>,
    per_page: Option<i64>,
}

#[get("/pois")]
async fn get_poi(pool: Data<PgPool>, query: Query<Pagination>) -> impl Responder {
    let pois = sqlx::query_as!(
        Poi,
        "
            SELECT *
            FROM pois
            LIMIT $1
            OFFSET $2
        ",
        query.per_page.unwrap_or(10),
        query.page.unwrap_or(0) * query.per_page.unwrap_or(10)
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
async fn create_poi(_pool: Data<PgPool>, _json: Json<Poi>) -> impl Responder {
    // let poi = sqlx::query_as!(
    //     Poi,
    //     "
    //         INSERT INTO pois (name, category_id, address, latitude, longitude)
    //         VALUES ($1, $2, $3, $4, $5)
    //         RETURNING *
    //     ",
    //     json.name,
    //     json.category_id,
    //     json.address,
    //     json.latitude,
    //     json.longitude,
    // )
    // .fetch_one(&**pool) // -> Poi
    // .await;
    HttpResponse::InternalServerError().body("tbd.".to_string())
    //HttpResponse::InternalServerError().body(format!("Failed to execute query: {:?}", e));
}
