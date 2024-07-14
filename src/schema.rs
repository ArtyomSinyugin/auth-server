// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

    tokens (token) {
        token -> Varchar,
        user_id -> Uuid,
        created_at -> Timestamp,
        last_used_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Uuid,
        #[max_length = 128]
        username -> Varchar,
        access_rights -> Int4,
        secret -> Text,
    }
}

diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tokens,
    users,
);
