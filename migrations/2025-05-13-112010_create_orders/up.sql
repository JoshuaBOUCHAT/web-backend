-- Your SQL goes here
CREATE TABLE orders(
   id_order INTEGER PRIMARY KEY AUTOINCREMENT,
   date_order TEXT NOT NULL,
   date_retrieve TEXT NOT NULL,
   id_user INTEGER NOT NULL,
   FOREIGN KEY(id_user) REFERENCES users(id_user)
);


