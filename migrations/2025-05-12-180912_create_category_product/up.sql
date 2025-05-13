-- Your SQL goes here
CREATE TABLE category_product(
   id_products INTEGER,
   id_categories INTEGER,
   PRIMARY KEY(id_products, id_categories),
   FOREIGN KEY(id_products) REFERENCES products(id_products),
   FOREIGN KEY(id_categories) REFERENCES categories(id_categories)
);
