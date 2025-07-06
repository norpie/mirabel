// @generated automatically by Diesel CLI.

diesel::table! {
    auth_options (id) {
        id -> Text,
        user_id -> Text,
        two_factor_encoded -> Nullable<Text>,
    }
}

diesel::table! {
    avatars (id) {
        id -> Text,
        user_id -> Text,
    }
}

diesel::table! {
    deleted_users (id) {
        id -> Text,
        deleted_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
    }
}

diesel::joinable!(auth_options -> users (user_id));
diesel::joinable!(avatars -> users (user_id));
diesel::joinable!(deleted_users -> users (id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_options,
    avatars,
    deleted_users,
    users,
);
