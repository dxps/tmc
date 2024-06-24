CREATE TABLE users_permissions
(
    user_id         CHAR(10)      NOT NULL,
    permission      VARCHAR(256)  NOT NULL,
    UNIQUE (user_id, permission),
    CONSTRAINT user_fk FOREIGN KEY(user_id) REFERENCES users_accounts(id)
);

