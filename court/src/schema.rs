// @generated automatically by Diesel CLI.

diesel::table! {
    recipes (id) {
        id -> Int4,
        title -> Text,
        instructions -> Text,
    }
}
