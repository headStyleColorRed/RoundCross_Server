-- Your SQL goes here
CREATE TABLE emergencies (
    uuid UUID PRIMARY KEY,
    active BOOLEAN NOT NULL DEFAULT 't'
);