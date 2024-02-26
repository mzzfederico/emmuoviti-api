-- CREATE EXTENSION postgis;

-- Create table for POI categories
CREATE TABLE poi_categories
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE poi_address (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    street VARCHAR(255),
    city VARCHAR(255),
    country VARCHAR(255),
    postcode VARCHAR(255),
    housenumber VARCHAR(255)
);

-- Create table for POIs
CREATE TABLE pois
(
    id          SERIAL PRIMARY KEY,
    slug        VARCHAR(255) UNIQUE NOT NULL,
    name        VARCHAR(255)     NOT NULL,
    category_id INTEGER REFERENCES poi_categories (id),
    address_id  INTEGER REFERENCES poi_address (id),
    latitude    DOUBLE PRECISION     NOT NULL,
    longitude   DOUBLE PRECISION     NOT NULL,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create table for POI metadata
CREATE TABLE poi_metadata
(
    id         SERIAL PRIMARY KEY,
    poi_id     INTEGER REFERENCES pois (id),
    key        VARCHAR(100) NOT NULL,
    value      TEXT         NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (poi_id, key)
);

-- Add indexes for faster utilities
CREATE INDEX idx_pois_category_id ON pois (category_id);
CREATE INDEX idx_pois_slug ON pois (slug);
CREATE INDEX idx_poi_metadata_poi_id ON poi_metadata (poi_id);
CREATE INDEX idx_poi_metadata_key ON poi_metadata (key);



