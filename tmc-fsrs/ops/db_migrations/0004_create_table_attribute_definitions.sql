CREATE TABLE attribute_definitions
(
    id              CHAR(10)      PRIMARY KEY,
    name            VARCHAR(64)   NOT NULL,
    description     VARCHAR(128),
    data_type       VARCHAR(16)   NOT NULL,
    default_value   VARCHAR(20),
    is_required     BOOLEAN       DEFAULT false,
    category_id     CHAR(10),
    CONSTRAINT category_fk FOREIGN KEY(category_id) REFERENCES attribute_categories(id)
);

