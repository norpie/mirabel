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
    users (id) {
        id -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}

diesel::joinable!(auth_options -> users (user_id));
diesel::joinable!(avatars -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_options,
    avatars,
    users,
);
