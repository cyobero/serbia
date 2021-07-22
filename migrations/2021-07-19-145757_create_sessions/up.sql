-- Your SQL goes here
CREATE TABLE sessions (
    session_key VARCHAR(255) NOT NULL,
    user_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (session_key),
    FOREIGN KEY (user_id) REFERENCES users (id)
);
