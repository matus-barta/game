-- Your SQL goes here
CREATE TYPE light_type AS ENUM('SPOT', 'OMNI');

CREATE TYPE color AS (r DECIMAL, g DECIMAL, b DECIMAL);

CREATE TYPE vec3 AS (x DECIMAL, y DECIMAL, z DECIMAL);

CREATE TYPE light_source AS (
    light_type LIGHT_TYPE,
    color COLOR,
    energy DECIMAL,
    light_size DECIMAL
);

CREATE TABLE
    models (
        id BIGSERIAL PRIMARY KEY,
        asset_id VARCHAR(64) NOT NULL,
        model_name VARCHAR NOT NULL
    );

CREATE TABLE
    placables (
        id BIGSERIAL PRIMARY KEY,
        model_id BIGINT NOT NULL REFERENCES models (id),
        light_source LIGHT_SOURCE,
    );

CREATE TABLE
    world_objects (
        id BIGSERIAL PRIMARY KEY,
        model_id BIGINT NOT NULL REFERENCES models (id),
    );

CREATE TABLE
    chunks (
        id BIGSERIAL PRIMARY KEY,
        model_id BIGINT NOT NULL REFERENCES models (id),
    );