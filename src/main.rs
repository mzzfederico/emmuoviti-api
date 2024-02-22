mod queries;
mod routes;

use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn handle_import() {
    let arguments: Vec<String> = std::env::args().collect();

    arguments.windows(2).for_each(|args| {
        if args[0] == "--import-geojson" {
            if let Ok(geo_json) = std::fs::read_to_string(&args[1]) {
                queries::import_geojson::import_geo_json(geo_json);
                println!("GeoJSON imported successfully");
                std::process::exit(0);
            } else {
                println!("Failed to read GeoJSON file");
                std::process::exit(1);
            }
        }
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    dotenv().ok();
    femme::start();

    handle_import();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

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
}
