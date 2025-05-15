-- Your SQL goes here
CREATE TABLE users(
   id_user INTEGER PRIMARY KEY AUTOINCREMENT,
   mail TEXT NOT NULL,
   phone_number TEXT NOT NULL,
   password_hash TEXT NOT NULL,
   date_creation TEXT NOT NULL,
   admin INTEGER NOT NULL DEFAULT 1,
   UNIQUE(mail)
);
