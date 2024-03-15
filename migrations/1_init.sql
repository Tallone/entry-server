-- Create the Users table
CREATE TABLE Users (
    user_id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create the Products table
CREATE TABLE licenses (
    license_id SERIAL PRIMARY KEY,
    license_key TEXT UNIQUE NOT NULL,
    status SMALLINT NOT NULL,
    valid_until TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create the ActivationLogs table
CREATE TABLE activations (
    activation_id SERIAL PRIMARY KEY,
    license_id INT NOT NULL,
    user_id UUID NOT NULL,
    device_id TEXT,
    ip_address TEXT NOT NULL,
    activation_date TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
