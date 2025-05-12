-- Your SQL goes here
CREATE TABLE category_product(
   Id_products INTEGER,
   Id_categories INTEGER,
   PRIMARY KEY(Id_products, Id_categories),
   FOREIGN KEY(Id_products) REFERENCES products(Id_products),
   FOREIGN KEY(Id_categories) REFERENCES categories(Id_categories)
);
