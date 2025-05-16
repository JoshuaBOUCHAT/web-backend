-- Your SQL goes here
CREATE TABLE categories(
   id_category INTEGER PRIMARY KEY AUTOINCREMENT,
   name TEXT NOT NULL,
   description TEXT NOT NULL,
   super_category INTEGER,
   FOREIGN KEY(super_category) REFERENCES categories(id_category)
);
