-- Your SQL goes here
CREATE TABLE users(
   id_users INTEGER,
   mail TEXT NOT NULL,
   phone_number TEXT NOT NULL,
   password_hash TEXT NOT NULL,
   date_creation NUMERIC,
   PRIMARY KEY(id_users)
);
DROP TABLE users;