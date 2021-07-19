-- Your SQL goes here
CREATE TABLE sessions (
    id INT NOT NULL AUTO_INCREMENT,
    user_id INT NOT NULL,
    begin TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    end TIMESTAMP NULL,
    actix_session VARCHAR(255) NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);
