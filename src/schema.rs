table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        post -> Text,
        author -> Int4,
        create_date -> Timestamptz,
        update_date -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        pass -> Varchar,
        create_date -> Timestamptz,
    }
}

joinable!(posts -> users (author));

allow_tables_to_appear_in_same_query!(posts, users,);
