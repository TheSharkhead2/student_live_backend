CREATE TABLE classes (
    id SERIAL PRIMARY KEY,
    code VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    questions TEXT[] NOT NULL,
    upvotes INTEGER[] NOT NULL
)