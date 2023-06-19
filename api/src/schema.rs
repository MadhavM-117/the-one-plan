// @generated automatically by Diesel CLI.

diesel::table! {
    auth_sessions (id) {
        id -> Text,
        user_id -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    files (id) {
        id -> Text,
        path -> Text,
    }
}

diesel::table! {
    rsvp_events (id) {
        id -> Text,
        name -> Text,
        venue -> Text,
        date_time -> Timestamp,
        webpage_id -> Text,
    }
}

diesel::table! {
    rsvp_sessions (id) {
        id -> Text,
        user_id -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    rsvp_users (id) {
        id -> Text,
        name -> Text,
        phone_number -> Text,
    }
}

diesel::table! {
    rsvps (id) {
        id -> Text,
        user_id -> Text,
        event_id -> Text,
        webpage_id -> Text,
        people_count -> Int4,
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

diesel::table! {
    webpages (id) {
        id -> Text,
        user_id -> Text,
        config_file -> Text,
        url -> Nullable<Text>,
        published_config_file -> Nullable<Text>,
    }
}

diesel::joinable!(auth_sessions -> users (user_id));
diesel::joinable!(rsvp_events -> webpages (webpage_id));
diesel::joinable!(rsvp_sessions -> rsvp_users (user_id));
diesel::joinable!(rsvps -> rsvp_events (event_id));
diesel::joinable!(rsvps -> rsvp_users (user_id));
diesel::joinable!(rsvps -> webpages (webpage_id));
diesel::joinable!(webpages -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_sessions,
    files,
    rsvp_events,
    rsvp_sessions,
    rsvp_users,
    rsvps,
    users,
    webpages,
);
