use std::fmt::Debug;
use std::ops::Add;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::models::{Poi, PoiAddress};

#[derive(Serialize, Deserialize, Debug)]
pub struct GeoJson {
    #[serde(rename = "type")]
    pub data_type: String,
    pub generator: String,
    pub copyright: String,
    pub timestamp: String,
    pub features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    #[serde(rename = "type")]
    pub feature_type: String,
    pub properties: Properties,
    pub geometry: Geometry,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    #[serde(rename = "@id")]
    pub id: String,
    pub amenity: Option<String>,
    pub created_by: Option<String>,
    pub name: Option<String>,
    pub cuisine: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub wheelchair: Option<String>,
    #[serde(rename = "addr:city")]
    pub addr_city: Option<String>,
    #[serde(rename = "addr:country")]
    pub addr_country: Option<String>,
    #[serde(rename = "addr:housenumber")]
    pub addr_housenumber: Option<String>,
    #[serde(rename = "addr:postcode")]
    pub addr_postcode: Option<String>,
    #[serde(rename = "addr:street")]
    pub addr_street: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub geometry_type: String,
    pub coordinates: Vec<f64>,
}

pub async fn import_geo_json(_geo_json: String, db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let features = serde_json::from_str::<GeoJson>(&_geo_json)?.features;

    for feature in features {
        let properties = feature.properties;

        let address: PoiAddress = sqlx::query_as!(
            PoiAddress,
            "
                INSERT INTO poi_address (street, city, country, postcode, housenumber)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING *
            ",
            properties.addr_street.unwrap_or("".to_string()),
            properties.addr_city.unwrap_or("".to_string()),
            "Italy",
            properties.addr_postcode.unwrap_or("".to_string()),
            properties.addr_housenumber.unwrap_or("".to_string())
        ).fetch_one(db_pool).await?;

        let longitude: f64 = feature.geometry.coordinates[0];
        let latitude: f64 = feature.geometry.coordinates[1];
        let name = properties.name.unwrap_or("Unknown".to_string());

        let unique_id = properties.id;
        let slug = name.to_lowercase().replace(' ', "-").add("-").add(&unique_id);

        // type geog not supported (yet!) by sqlx for now
        // reverting to querying directly
        let _poi = sqlx::query_as!(
            Poi,
            "
                INSERT INTO pois (name, category_id, address_id, latitude, longitude, slug)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING *
            ",
            name,
            1,
            address.id,
            latitude,
            longitude,
            slug
        ).fetch_one(db_pool).await?;

        println!("{:?}", _poi);
    }

    Ok(())
}
