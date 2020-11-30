use super::schema::ingredients;
use crate::schema::*;
use crate::unit::Unit;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Identifiable, Associations, Serialize)]
pub struct Ingredient {
    id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "ingredients"]
pub struct NewIngredient {
    pub name: String,
}

#[derive(Insertable, Queryable, Identifiable, Associations)]
#[table_name = "ingredient_lists"]
#[belongs_to(Ingredient)]
#[belongs_to(Recipe)]
pub struct IngredientListItem {
    id: i32,
    recipe_id: i32,
    ingredient_id: i32,
    quantity: f32,
    unit: Unit,
}

#[derive(Insertable, Queryable, Identifiable, Associations)]
pub struct Recipe {
    id: i32,
    name: String,
}
