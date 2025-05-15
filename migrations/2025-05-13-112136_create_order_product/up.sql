-- Your SQL goes here
CREATE TABLE order_product(
   id_product INTEGER,
   id_order INTEGER,
   quantity INTEGER NOT NULL,
   PRIMARY KEY(id_product, id_order),
   FOREIGN KEY(id_product) REFERENCES products(id_product),
   FOREIGN KEY(id_order) REFERENCES orders(id_order)
);
