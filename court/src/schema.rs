// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        expires -> Int4,
        jwt_hash -> Text,
        jwt -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        auth0subject -> Text,
        name -> Text,
    }
}

diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(sessions, users,);
