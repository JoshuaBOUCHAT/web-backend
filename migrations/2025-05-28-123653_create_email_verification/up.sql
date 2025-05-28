-- Your SQL goes here
CREATE TABLE email_verifications (
    id_email_verification INTEGER PRIMARY KEY AUTOINCREMENT,
    id_user               INTEGER NOT NULL,
    expiration            TEXT    NOT NULL,
    token                 TEXT    NOT NULL,
    FOREIGN KEY (id_user) REFERENCES users(id_users)
);