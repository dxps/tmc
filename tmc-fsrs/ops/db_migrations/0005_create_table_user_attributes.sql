CREATE TABLE user_attributes
(
    id              CHAR(10)      PRIMARY KEY,
    did             CHAR(10),
    uid             CHAR(10),
    name            VARCHAR(64)   NOT NULL,
    value           VARCHAR(1024),
    CONSTRAINT definition_fk FOREIGN KEY(did) REFERENCES attribute_definitions(id)
);

COMMENT ON COLUMN user_attributes.did is 'The definition id of this attribute instance.';
COMMENT ON COLUMN user_attributes.uid is 'The user id that has this attribute instance.';
