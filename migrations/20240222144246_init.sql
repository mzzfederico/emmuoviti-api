-- Create table for POI categories
CREATE TABLE poi_categories
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

-- Create table for POIs
CREATE TABLE pois
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(255)     NOT NULL,
    category_id INTEGER REFERENCES poi_categories (id),
    address     VARCHAR(255)     NOT NULL,
    latitude    DOUBLE PRECISION NOT NULL,
    longitude   DOUBLE PRECISION NOT NULL,
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

-- Add indexes for faster queries
CREATE INDEX idx_pois_category_id ON pois (category_id);
CREATE INDEX idx_pois_location ON pois (latitude, longitude);

