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

table! {
    times (date) {
        date -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    airports,
    carriers,
    times,
);
