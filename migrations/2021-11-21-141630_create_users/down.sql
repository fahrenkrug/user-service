-- This file should undo anything in `up.sql`
CREATE TABLE backup_2021_11_21_141630_create_users (
    id uuid NOT NULL,
   first_name VARCHAR NOT NULL,
   last_name VARCHAR NOT NULL,
   created_at timestamptz NOT NULL,
   updated_at timestamptz NOT NULL,
   PRIMARY KEY (id)
);

CREATE TABLE backup_2021_11_21_141630_create_email_identities (
   id uuid NOT NULL,
   user_id uuid NOT NULL,
   email VARCHAR NOT NULL,
   hash VARCHAR NOT NULL,
   created_at timestamptz NOT NULL DEFAULT NOW(),
   updated_at timestamptz NOT NULL DEFAULT NOW(),
   PRIMARY KEY (id)
);

INSERT INTO backup_2021_11_21_141630_create_users (SELECT * FROM users);
INSERT INTO backup_2021_11_21_141630_create_email_identities (SELECT * FROM email_identities);

DROP TABLE users;
DROP TABLE email_identities;