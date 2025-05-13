-- Your SQL goes here
CREATE TABLE order_product(
   id_orders INTEGER,
   id_products INTEGER,
   nombre INTEGER,
   PRIMARY KEY(id_orders, id_products),
   FOREIGN KEY(id_orders) REFERENCES orders(id_orders),
   FOREIGN KEY(id_products) REFERENCES products(id_products)
);
