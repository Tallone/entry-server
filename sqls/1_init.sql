-- Create the Users table
CREATE TABLE Users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(20),
    email TEXT UNIQUE NOT NULL,
    password TEXT,
    status SMALLINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create the Products table
CREATE TABLE licenses (
    id SERIAL PRIMARY KEY,
    key TEXT UNIQUE NOT NULL,
    status SMALLINT NOT NULL,
    valid_until TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL  DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL  DEFAULT CURRENT_TIMESTAMP
);

-- Create the ActivationLogs table
CREATE TABLE activations (
    id SERIAL PRIMARY KEY,
    license_id INT NOT NULL,
    user_id UUID NOT NULL,
    device_id TEXT,
    ip_address TEXT NOT NULL,
    activation_date TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
