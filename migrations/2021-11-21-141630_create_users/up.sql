CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users (
    id uuid DEFAULT uuid_generate_v4(),
    email VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id)
);


CREATE OR REPLACE FUNCTION import_users_if_exists() RETURNS VOID AS $$
BEGIN
    INSERT INTO "users" (SELECT * from migration_create_users_backup_users);
    DROP TABLE migration_create_users_backup_users;
EXCEPTION
    WHEN undefined_table THEN
    RETURN;
END;
$$ LANGUAGE plpgsql;

SELECT import_users_if_exists();

SELECT diesel_manage_updated_at('users');