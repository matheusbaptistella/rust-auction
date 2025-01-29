-- migrations/20250129003359_create_users_table.sql

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_name VARCHAR(64) NOT NULL,
    email VARCHAR(254) NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL,
    salt BYTEA NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
