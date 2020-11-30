table! {
    use diesel::sql_types::*;
    use crate::unit::*;

    ingredient_lists (id) {
        id -> Int4,
        recipe_id -> Int4,
        ingredient_id -> Int4,
        quantity -> Float4,
        unit -> UnitMapping,
    }
}

table! {
    use diesel::sql_types::*;

    ingredients (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;

    recipes (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(ingredient_lists -> ingredients (ingredient_id));
joinable!(ingredient_lists -> recipes (recipe_id));

allow_tables_to_appear_in_same_query!(ingredient_lists, ingredients, recipes,);
