table! {
    users (id) {
        id -> Int4,
        firstname -> Text,
        lastname -> Text,
        email -> Text,
        password_digest -> Text,
        created_at -> Timestamp,
    }
}
