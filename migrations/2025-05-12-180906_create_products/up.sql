-- Your SQL goes here
CREATE TABLE products (
    id_product INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    image_url TEXT,
    visible NOT NULL DEFAULT 1
);