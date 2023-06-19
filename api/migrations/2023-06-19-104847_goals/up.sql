-- Your SQL goes here
CREATE TABLE goals (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE action_points (
    id TEXT PRIMARY KEY NOT NULL,
    goal_id TEXT NOT NULL REFERENCES goals(id) ON DELETE CASCADE,
    completed BOOLEAN NOT NULL,
    action_point_type TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
