CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
create table yolo_message (
    id serial primary key,
    uuid1 uuid not null,
    username varchar not null,
    cipherText varchar not null,
    key varchar not null,
    nonce varchar not null,
    conversationName varchar not null,
    date varchar not null
);