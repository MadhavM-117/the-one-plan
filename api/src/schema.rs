// @generated automatically by Diesel CLI.

diesel::table! {
    action_points (id) {
        id -> Text,
        goal_id -> Text,
        completed -> Bool,
        action_point_type -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    auth_sessions (id) {
        id -> Text,
        user_id -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    goals (id) {
        id -> Text,
        user_id -> Text,
        title -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        phone_number -> Nullable<Text>,
        password -> Text,
    }
}

diesel::joinable!(action_points -> goals (goal_id));
diesel::joinable!(auth_sessions -> users (user_id));
diesel::joinable!(goals -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    action_points,
    auth_sessions,
    goals,
    users,
);
