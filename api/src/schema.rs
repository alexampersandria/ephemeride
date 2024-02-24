// @generated automatically by Diesel CLI.

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
