-- Your SQL goes here
CREATE TABLE orders(
   id_orders INTEGER,
   date_order TEXT NOT NULL,
   date_retrieve TEXT NOT NULL,
   id_users INTEGER NOT NULL,
   PRIMARY KEY(id_orders),
   FOREIGN KEY(id_users) REFERENCES users(id_users)
);


