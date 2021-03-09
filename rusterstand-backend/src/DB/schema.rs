table! {
    users (id) {
        id -> Nullable<Integer>,
        firstName -> Text,
        lastName -> Text,
        email -> Text,
        password_digest -> Text,
        created_at -> Timestamp,
    }
}