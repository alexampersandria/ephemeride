// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        created_at -> Int8,
    }
}

diesel::table! {
    entries (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        date -> Varchar,
        created_at -> Int8,
        mood -> Int4,
        #[max_length = 1023]
        entry -> Nullable<Varchar>,
    }
}

diesel::table! {
    entry_tags (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        entry_id -> Varchar,
        #[max_length = 255]
        tag_id -> Varchar,
    }
}

diesel::table! {
    invites (id) {
        #[max_length = 255]
        id -> Varchar,
        created_at -> Int8,
        #[max_length = 255]
        code -> Varchar,
        used -> Bool,
    }
}

diesel::table! {
    sessions (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        created_at -> Int8,
        accessed_at -> Int8,
        #[max_length = 255]
        ip_address -> Varchar,
        #[max_length = 255]
        user_agent -> Varchar,
    }
}

diesel::table! {
    tags (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        created_at -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        color -> Varchar,
        #[max_length = 255]
        category_id -> Varchar,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        created_at -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        invite -> Nullable<Varchar>,
    }
}

diesel::joinable!(categories -> users (user_id));
diesel::joinable!(entries -> users (user_id));
diesel::joinable!(entry_tags -> entries (entry_id));
diesel::joinable!(entry_tags -> tags (tag_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(tags -> categories (category_id));
diesel::joinable!(tags -> users (user_id));
diesel::joinable!(users -> invites (invite));

diesel::allow_tables_to_appear_in_same_query!(
  categories, entries, entry_tags, invites, sessions, tags, users,
);
