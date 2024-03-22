CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(150) NOT NULL,
    name VARCHAR(180) NOT NULL,
    email VARCHAR(300) NOT NULL,
    password VARCHAR(200) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    is_adm BOOLEAN DEFAULT FALSE
);

CREATE INDEX idx_id ON users (id);
