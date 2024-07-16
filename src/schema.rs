// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

    jobs (job, user_id) {
        #[max_length = 128]
        job -> Varchar,
        user_id -> Uuid,
        created_at -> Timestamp,
        last_used_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    timers (idx) {
        idx -> Varchar,
        user_id -> Uuid,
        #[max_length = 128]
        job -> Varchar,
        date -> Date,
        #[max_length = 128]
        started_at -> Varchar,
        #[max_length = 128]
        finished_at -> Nullable<Varchar>,
    }
}

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
        #[max_length = 255]
        company -> Nullable<Varchar>,
        secret -> Text,
    }
}

diesel::joinable!(jobs -> users (user_id));
diesel::joinable!(timers -> users (user_id));
diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    jobs,
    timers,
    tokens,
    users,
);
