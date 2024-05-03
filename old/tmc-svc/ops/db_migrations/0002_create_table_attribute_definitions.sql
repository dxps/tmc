CREATE TABLE attribute_definitions
(
    id VARCHAR(21) PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    description VARCHAR(128),
    data_type VARCHAR(16) NOT NULL
);