table! {
    reset (token) {
        token -> Varchar,
        email -> Varchar,
    }
}

table! {
    user_token (user_id) {
        user_id -> Uuid,
        token -> Varchar,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(user_token -> users (user_id));

allow_tables_to_appear_in_same_query!(
    reset,
    user_token,
    users,
);
