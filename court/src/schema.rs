// @generated automatically by Diesel CLI.

diesel::table! {
    recipes (id) {
        id -> Int4,
        title -> Text,
        instructions -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        auth0subject -> Text,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    recipes,
    users,
);
