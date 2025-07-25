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
    jobs (id) {
        id -> Text,
        session_id -> Text,
        parent_job_id -> Nullable<Text>,
        job_type -> Int4,
        status -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    prompt_evaluations (id) {
        id -> Text,
        job_id -> Text,
        prompt_token_count -> Int4,
        response_token_count -> Int4,
        evaluation_start -> Timestamptz,
        evaluation_end -> Timestamptz,
    }
}

diesel::table! {
    sessions (id) {
        id -> Text,
        workspace_id -> Text,
        user_id -> Text,
        title -> Text,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
        archived -> Bool,
    }
}

diesel::table! {
    timeline_entries (id) {
        id -> Text,
        session_id -> Text,
        content -> Jsonb,
        created_at -> Timestamptz,
        content_type -> Text,
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

diesel::table! {
    workspace_members (id) {
        id -> Text,
        workspace_id -> Text,
        user_id -> Text,
        role -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    workspaces (id) {
        id -> Text,
        name -> Text,
        avatar -> Nullable<Text>,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
    }
}

diesel::joinable!(auth_options -> users (user_id));
diesel::joinable!(avatars -> users (user_id));
diesel::joinable!(deleted_users -> users (id));
diesel::joinable!(jobs -> sessions (session_id));
diesel::joinable!(prompt_evaluations -> jobs (job_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(sessions -> workspaces (workspace_id));
diesel::joinable!(timeline_entries -> sessions (session_id));
diesel::joinable!(workspace_members -> users (user_id));
diesel::joinable!(workspace_members -> workspaces (workspace_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_options,
    avatars,
    deleted_users,
    jobs,
    prompt_evaluations,
    sessions,
    timeline_entries,
    users,
    workspace_members,
    workspaces,
);
