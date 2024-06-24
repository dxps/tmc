
CREATE TABLE user_accounts (
    id              CHAR(10)                 PRIMARY KEY,
    email           VARCHAR(64)                           DEFAULT '' UNIQUE,
    username        VARCHAR(48)              NOT NULL,
    created_at      TIMESTAMP WITH TIME ZONE              DEFAULT current_timestamp,
    updated_at      TIMESTAMP WITH TIME ZONE              DEFAULT current_timestamp,
    is_anonymous    BOOLEAN                  NOT NULL     DEFAULT FALSE,
    password        VARCHAR(255)             NOT NULL,
    salt            CHAR(12)                 NOT NULL,
    bio             TEXT                                  DEFAULT '',
    image           VARCHAR(512),
    state           CHAR(1)                  NOT NULL     DEFAULT 'A'
);

COMMENT ON COLUMN user_accounts.image is 'The Base64 encoded version of the user avatar.';

INSERT INTO user_accounts (id, username, is_anonymous, password, salt)
            VALUES ('j1-SOTHHxi', 'guest', true, '', '');
