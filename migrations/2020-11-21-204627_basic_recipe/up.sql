-- Your SQL goes here
CREATE TYPE UNIT as ENUM ('g', 'ml', 'ts', 'tbls');

CREATE TABLE ingredient (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE recipe (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE ingredient_list (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipe(id),
    ingredient INTEGER NOT NULL REFERENCES ingredient(id),
    quantity REAL NOT NULL,
    unit UNIT NOT NULL
);
