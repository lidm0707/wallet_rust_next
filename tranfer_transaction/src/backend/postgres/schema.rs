// @generated automatically by Diesel CLI.

diesel::table! {
    trans (id) {
        id -> Int4,
        sender -> Int4,
        receiver -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        amount -> Float8,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        amount -> Float8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    trans,
    users,
);
