mod utilities;
mod routes;
mod models;

use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{PgPool, Pool, Postgres};
use std::env;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn handle_import(pool: Pool<Postgres>) -> anyhow::Result<()> {
    let arguments: Vec<String> = std::env::args().collect();
    
    if arguments.len() < 2 {
        return Ok(());
    }

    println!("{:?}", arguments);

    if arguments[1] == "--import-geojson" {
        println!("Importing GeoJSON");
        if let Ok(geo_json) = std::fs::read_to_string(&arguments[2]) {
            println!("GeoJSON file read successfully");
            utilities::import_geo_json::import_geo_json(geo_json, &pool).await.map_err(anyhow::Error::from)?;
            println!("GeoJSON imported successfully");
        } else {
            println!("Failed to read GeoJSON file");
        }
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    dotenv().ok();
    femme::start();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    handle_import(pool.clone()).await?;

    println!("Starting server");
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(routes::pois::get_poi)
            .service(routes::pois::create_poi)
            .service(routes::pois::get_poi_by_radius)
            .route("/", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(anyhow::Error::from)
}
