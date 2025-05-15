-- Your SQL goes here
CREATE TABLE categories(
   id_category INTEGER PRIMARY KEY AUTOINCREMENT,
   name TEXT NOT NULL,
   description TEXT,
   id_category_1 INTEGER,
   FOREIGN KEY(id_category_1) REFERENCES categories(id_category)
);
