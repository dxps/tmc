
CREATE TABLE users_accounts (
    id              BIGSERIAL                PRIMARY KEY,
    email           VARCHAR(255)                          DEFAULT '' UNIQUE,
    username        VARCHAR(255)             NOT NULL,
    created_at      TIMESTAMP WITH TIME ZONE              DEFAULT current_timestamp,
    updated_at      TIMESTAMP WITH TIME ZONE              DEFAULT current_timestamp,
    anonymous       BOOLEAN                  NOT NULL     DEFAULT FALSE,
    password        VARCHAR(255)             NOT NULL,
    salt            CHAR(12)                 NOT NULL,
    bio             TEXT                                  DEFAULT '',
    image           VARCHAR(255),
    state           CHAR(1)                  NOT NULL     DEFAULT 'A'
);

INSERT INTO users_accounts (id, username, anonymous, password, salt)
            VALUES (1, 'guest', true, '', '');
