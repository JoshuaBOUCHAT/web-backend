-- Your SQL goes here
CREATE TABLE category_product(
   id_category INTEGER ,
   id_product INTEGER,
   PRIMARY KEY(id_category, id_product),
   FOREIGN KEY(id_category) REFERENCES categories(id_category),
   FOREIGN KEY(id_product) REFERENCES products(id_product)
);
