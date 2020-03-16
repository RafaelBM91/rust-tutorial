CREATE TABLE IF NOT EXISTS contacts (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    last_name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS personals (
    id VARCHAR PRIMARY KEY,
    contact_id VARCHAR REFERENCES contacts(id),
    cellphone VARCHAR NOT NULL,
    email VARCHAR,
    address VARCHAR
);

CREATE TABLE IF NOT EXISTS companies (
    id VARCHAR PRIMARY KEY,
    contact_id VARCHAR REFERENCES contacts(id),
    phone VARCHAR NOT NULL,
    email VARCHAR,
    address VARCHAR
);

-- ------------------------------ --

CREATE TABLE IF NOT EXISTS models (
    id VARCHAR(100) PRIMARY KEY,
    name VARCHAR(60) NOT NULL,
    age INT NOT NULL
);

CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
);
