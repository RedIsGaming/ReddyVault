diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        password -> Timestamp,
        created -> Timestamp,
    }
}
