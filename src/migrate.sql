CREATE TABLE models (
    id VARCHAR(100) PRIMARY KEY,
    name VARCHAR(60) NOT NULL,
    age INT NOT NULL
);

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
)
