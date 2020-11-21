table! {
    ingredient (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    ingredient_list (id) {
        id -> Int4,
        recipe_id -> Int4,
        ingredient -> Int4,
        quantity -> Float4,
        unit -> Unit,
    }
}

table! {
    recipe (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(ingredient_list -> ingredient (ingredient));
joinable!(ingredient_list -> recipe (recipe_id));

allow_tables_to_appear_in_same_query!(
    ingredient,
    ingredient_list,
    recipe,
);
