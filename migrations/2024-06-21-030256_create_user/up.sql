DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role_type') THEN
        CREATE TYPE role_type AS ENUM (
            'admin',
            'customer'
        );
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  email VARCHAR(100) NOT NULL UNIQUE,
  verified BOOLEAN NOT NULL DEFAULT FALSE,
  password VARCHAR(100) NOT NULL,
  role role_type NOT NULL DEFAULT 'customer'
);

CREATE INDEX IF NOT EXISTS users_email_idx ON users (email);
