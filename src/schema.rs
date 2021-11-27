table! {
    email_identities (id) {
        id -> Uuid,
        user_id -> Uuid,
        email -> Varchar,
        hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    email_identities,
    users,
);
