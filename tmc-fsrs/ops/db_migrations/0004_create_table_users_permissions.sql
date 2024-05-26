CREATE TABLE users_permissions (
    user_id         BIGSERIAL                NOT NULL,
    permission      VARCHAR(256)             NOT NULL,
    UNIQUE (user_id, permission)
);
