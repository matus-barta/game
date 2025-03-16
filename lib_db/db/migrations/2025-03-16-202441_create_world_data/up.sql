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
        id VARCHAR(64) PRIMARY KEY,
        model_name VARCHAR NOT NULL
    );

CREATE TABLE
    placables (
        id BIGSERIAL PRIMARY KEY,
        model_id VARCHAR(64) NOT NULL REFERENCES models (id),
        position VEC3 NOT NULL,
        rotation VEC3 NOT NULL,
        light_source LIGHT_SOURCE,
        parent_id BIGINT NOT NULL
    );

CREATE TABLE
    world_objects (
        id BIGSERIAL PRIMARY KEY,
        model_id VARCHAR(64) NOT NULL REFERENCES models (id),
        position VEC3 NOT NULL,
        rotation VEC3 NOT NULL
    );

CREATE TABLE
    chunks (
        id BIGSERIAL PRIMARY KEY,
        model_id VARCHAR(64) NOT NULL
    );

ALTER TABLE placables
ADD FOREIGN KEY (parent_id) REFERENCES world_objects (id);

ALTER TABLE IF EXISTS placables
ADD FOREIGN KEY (parent_id) REFERENCES chunks (id);