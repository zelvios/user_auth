CREATE TABLE roles
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL CHECK (name ~* '^[a-z_]+$'
) ,
    description VARCHAR(255),
    permission BIGINT NOT NULL DEFAULT 0
);

INSERT INTO roles (name, description, permission)
VALUES ('admin', 'User with elevated privileges', 7246),
       ('developer', 'Application developer', 2096383),
       ('owner', 'Application owner', 2097151);

-- See access_control.md for detailed information about roles