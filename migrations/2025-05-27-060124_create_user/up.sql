CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users
(
    id            SERIAL PRIMARY KEY,
    temp_id       UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4(),
    email         VARCHAR(255) NOT NULL,
    username      VARCHAR(200) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name    VARCHAR(100) NOT NULL,
    last_name     VARCHAR(100) NOT NULL,
    is_active     BOOLEAN      NOT NULL DEFAULT TRUE,
    roles         SMALLINT     NOT NULL DEFAULT 0,
    permissions   BIGINT       NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ           DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMPTZ           DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX unique_active_email ON users (email) WHERE is_active = TRUE;

CREATE UNIQUE INDEX unique_active_username ON users (username) WHERE is_active = TRUE;


INSERT INTO users (email,
                   username,
                   password_hash,
                   first_name,
                   last_name,
                   is_active,
                   roles,
                   permissions)
VALUES ('owner@example.com',
        'owner',
        '$argon2id$v=19$m=19456,t=2,p=1$QjRpWHmbR0mqa68JLVjj6A$cbK74rWWj4GLAGRJvx4HKiGXndDoyn0mYH4u3z1FMGk', -- owner_password
        'OwnerFirstName',
        'OwnerLastName',
        TRUE,
        4,
        0);

-- See access_control.md for detailed information about users