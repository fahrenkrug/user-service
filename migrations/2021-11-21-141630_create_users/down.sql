-- This file should undo anything in `up.sql`
CREATE TABLE migration_create_users_backup_users (
    id uuid NOT NULL,
   email VARCHAR NOT NULL,
   first_name VARCHAR NOT NULL,
   last_name VARCHAR NOT NULL,
   created_at TIMESTAMP NOT NULL,
   updated_at TIMESTAMP NOT NULL,
   PRIMARY KEY (id)
);

INSERT INTO migration_create_users_backup_users (SELECT * FROM users);

DROP TABLE users;