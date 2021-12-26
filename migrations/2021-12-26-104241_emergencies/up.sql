-- Your SQL goes here

CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR NOT NULL,
    username VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    biking_modality VARCHAR NOT NULL
);

CREATE TABLE emergencies (
    id UUID PRIMARY KEY,
    active BOOLEAN NOT NULL DEFAULT 't',
    owner_id UUID NOT NULL,
    emergency_type VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL,
    localization VARCHAR NOT NULL,
    helped BOOLEAN NOT NULL DEFAULT 'f',
    FOREIGN KEY (owner_id) REFERENCES users(id)
);

CREATE TABLE answers (
    id UUID PRIMARY KEY,
    parent_id UUID NOT NULL,
    answer TEXT NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES emergencies(id)
);

CREATE TABLE messages (
    id UUID PRIMARY KEY,
    parent_id UUID NOT NULL,
    message_text TEXT NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES emergencies(id)
);