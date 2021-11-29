SET timezone TO 'UTC';
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users (
    id uuid DEFAULT uuid_generate_v4(),
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id)
);

CREATE TABLE email_identities (
    id uuid DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL,
    email VARCHAR NOT NULL UNIQUE ,
    hash VARCHAR NOT NULL,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id)
);

CREATE INDEX ON email_identities ((lower(email)));

CREATE OR REPLACE FUNCTION import_backup_2021_11_21_141630_if_exists() RETURNS VOID AS $$
BEGIN
    INSERT INTO "users" (SELECT * from backup_2021_11_21_141630_create_users);
    DROP TABLE backup_2021_11_21_141630_create_users;
    INSERT INTO "email_identities" (SELECT * from backup_2021_11_21_141630_create_email_identities);
    DROP TABLE backup_2021_11_21_141630_create_email_identities;
EXCEPTION
    WHEN undefined_table THEN
    RETURN;
END;
$$ LANGUAGE plpgsql;

SELECT import_backup_2021_11_21_141630_if_exists();

SELECT diesel_manage_updated_at('users');
SELECT diesel_manage_updated_at('email_identities');