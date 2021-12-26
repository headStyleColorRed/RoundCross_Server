-- Your SQL goes here

CREATE TABLE owners (
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
    helped BOOLEAN NOT NULL DEFAULT 'f',
    foreign key (owner_id) references owners(id)
);