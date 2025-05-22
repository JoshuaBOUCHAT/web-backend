-- Your SQL goes here
CREATE TABLE orders(
   id_order INTEGER PRIMARY KEY AUTOINCREMENT,
   date_order TEXT,
   date_retrieve TEXT,
   id_user INTEGER NOT NULL,
   FOREIGN KEY(id_user) REFERENCES users(id_user)
);


