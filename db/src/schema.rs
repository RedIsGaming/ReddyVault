diesel::table! {
    users (id) {
        id -> Oid,
        username -> Text,
        email -> Text,
        password -> Text,
        created -> Date,
        role -> Text,
    }
}
