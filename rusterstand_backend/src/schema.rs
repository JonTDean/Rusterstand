table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password_digest -> Varchar,
        created_at -> Timestamp,
    }
}
