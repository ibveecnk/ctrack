diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password -> Text,
        unique_id -> Text,
    }
}
