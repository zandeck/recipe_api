-- Your SQL goes here
CREATE TYPE UNIT as ENUM ('g', 'ml', 'ts', 'tbls');

CREATE TABLE ingredients (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE recipes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE ingredient_lists (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipes(id),
    ingredient_id INTEGER NOT NULL REFERENCES ingredients(id),
    quantity REAL NOT NULL,
    unit UNIT NOT NULL
);
