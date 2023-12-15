// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        active -> Bool,
        #[max_length = 30]
        username -> Varchar,
        #[max_length = 254]
        email -> Varchar,
        #[max_length = 60]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
