table! {
    airports (code) {
        code -> Text,
        name -> Text,
    }
}

table! {
    carriers (code) {
        code -> Text,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    airports,
    carriers,
);
