CREATE
OR REPLACE FUNCTION CURRENT_MS() RETURNS BIGINT AS $ $ BEGIN RETURN EXTRACT(
    EPOCH
    FROM
        now()
) * 1000;

END;

$ $ LANGUAGE plpgsql;

-- Create the Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(20),
    email TEXT UNIQUE NOT NULL,
    password TEXT,
    state SMALLINT NOT NULL DEFAULT 0,
    created_at BIGINT NOT NULL DEFAULT CURRENT_MS(),
    updated_at BIGINT NOT NULL DEFAULT CURRENT_MS()
);

-- Create the Products table
CREATE TABLE IF NOT EXISTS licenses (
    id SERIAL PRIMARY KEY,
    key TEXT UNIQUE NOT NULL,
    is_used BOOLEAN NOT NULL DEFAULT false,
    expired_at BIGINT,
    created_at BIGINT NOT NULL DEFAULT CURRENT_MS(),
    updated_at BIGINT NOT NULL DEFAULT CURRENT_MS()
);

-- Create the ActivationLogs table
CREATE TABLE IF NOT EXISTS activations (
    id SERIAL PRIMARY KEY,
    license_key TEXT NOT NULL,
    user_id UUID NOT NULL,
    device_id TEXT,
    ip_address TEXT NOT NULL,
    created_at BIGINT NOT NULL DEFAULT CURRENT_MS()
);

CREATE INDEX idx_uid_lid ON activations (user_id, license_key);

CREATE TABLE IF NOT EXISTS synchronize (
    user_id UUID PRIMARY KEY NOT NULL,
    version INT NOT NULL,
    content TEXT NOT NULL,
    created_at BIGINT NOT NULL DEFAULT CURRENT_MS(),
    updated_at BIGINT NOT NULL DEFAULT CURRENT_MS()
);