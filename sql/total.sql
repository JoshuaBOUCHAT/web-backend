/* ------------------------------------------------------------------ */
/* 1. CATÉGORIES (parents)                                            */
/* ------------------------------------------------------------------ */
INSERT INTO categories (name, description, super_category)
VALUES
  ('Viennoiseries', 'Baked goods typically eaten for breakfast', NULL),
  ('Patisseries'  , 'Sweet baked desserts'                       , NULL);

/* ------------------------------------------------------------------ */
/* 2. SOUS-CATÉGORIES                                                 */
/* ------------------------------------------------------------------ */
/* Viennoiseries */
INSERT INTO categories (name, description, super_category)
VALUES
  ('Pain au chocolat', 'Chocolate-filled puff pastry'          ,
     (SELECT id_category FROM categories WHERE name = 'Viennoiseries')),
  ('Croissant'       , 'Flaky buttery pastry'                  ,
     (SELECT id_category FROM categories WHERE name = 'Viennoiseries')),
  ('Pain au raisin'  , 'Spiral pastry with raisins and custard',
     (SELECT id_category FROM categories WHERE name = 'Viennoiseries'));

/* Patisseries */
INSERT INTO categories (name, description, super_category)
VALUES
  ('Macaron'        , 'French almond meringue cookie with ganache',
     (SELECT id_category FROM categories WHERE name = 'Patisseries')),
  ('Tarte au citron', 'Lemon tart with sweet pastry crust'        ,
     (SELECT id_category FROM categories WHERE name = 'Patisseries')),
  ('Éclair'         , 'Choux pastry filled with cream and icing'  ,
     (SELECT id_category FROM categories WHERE name = 'Patisseries'));

/* ------------------------------------------------------------------ */
/* 3. PRODUITS                                                        */
/* ------------------------------------------------------------------ */
INSERT INTO products (description, name, price, image_url) VALUES
('Description du produit 1', 'Produit 1', 10.99 , '/public/images/illustrator.svg'),
('Description du produit 2', 'Produit 2', 20.99 , '/public/images/illustrator.svg'),
('Description du produit 3', 'Produit 3', 30.99 , '/public/images/illustrator.svg'),
('Description du produit 4', 'Produit 4', 40.99 , '/public/images/illustrator.svg'),
('Description du produit 5', 'Produit 5', 50.99 , '/public/images/illustrator.svg'),
('Description du produit 6', 'Produit 6', 60.99 , '/public/images/illustrator.svg'),
('Description du produit 7', 'Produit 7', 70.99 , '/public/images/illustrator.svg'),
('Description du produit 8', 'Produit 8', 80.99 , '/public/images/illustrator.svg'),
('Description du produit 9', 'Produit 9', 90.99 , '/public/images/illustrator.svg'),
('Description du produit 10','Produit 10',100.99, '/public/images/illustrator.svg');

/* ------------------------------------------------------------------ */
/* 4. ASSOCIATIONS PRODUIT  ↔  (SOUS-)CATÉGORIE                       */
/* ------------------------------------------------------------------ */
/* Pour rester indépendants des ID auto-incrémentés, on fait tout      */
/* avec des sous-requêtes basées sur le nom.                           */

/* Produits rattachés aux viennoiseries */
INSERT INTO category_product (id_category, id_product) VALUES
  ((SELECT id_category FROM categories WHERE name = 'Pain au chocolat'), (SELECT id_product FROM products WHERE name = 'Produit 1')),
  ((SELECT id_category FROM categories WHERE name = 'Croissant')       , (SELECT id_product FROM products WHERE name = 'Produit 2')),
  ((SELECT id_category FROM categories WHERE name = 'Pain au raisin')  , (SELECT id_product FROM products WHERE name = 'Produit 3')),
  /* Produit 4 : viennoiserie générique */
  ((SELECT id_category FROM categories WHERE name = 'Viennoiseries')   , (SELECT id_product FROM products WHERE name = 'Produit 4')),
  /* Produit 9 : assortiment viennoiseries */
  ((SELECT id_category FROM categories WHERE name = 'Viennoiseries')   , (SELECT id_product FROM products WHERE name = 'Produit 9'));

/* Produits rattachés aux pâtisseries */
INSERT INTO category_product (id_category, id_product) VALUES
  ((SELECT id_category FROM categories WHERE name = 'Macaron')         , (SELECT id_product FROM products WHERE name = 'Produit 5')),
  ((SELECT id_category FROM categories WHERE name = 'Tarte au citron') , (SELECT id_product FROM products WHERE name = 'Produit 6')),
  ((SELECT id_category FROM categories WHERE name = 'Éclair')          , (SELECT id_product FROM products WHERE name = 'Produit 7')),
  /* Produit 8 : assortiment pâtisseries */
  ((SELECT id_category FROM categories WHERE name = 'Patisseries')     , (SELECT id_product FROM products WHERE name = 'Produit 8')),
  /* Produit 10 : plateau mixte pâtisserie & viennoiserie */
  ((SELECT id_category FROM categories WHERE name = 'Patisseries')     , (SELECT id_product FROM products WHERE name = 'Produit 10')),
  ((SELECT id_category FROM categories WHERE name = 'Viennoiseries')   , (SELECT id_product FROM products WHERE name = 'Produit 10'));

  INSERT INTO users (mail,phone_number,password_hash,date_creation, admin) VALUES
("joshuabouchat@gmail.com","0783232757","$argon2id$v=19$m=19456,t=2,p=1$N//bAQZaSmOZiO6RoALLmw$jQK97KaEAr3c881uk01lP74vr85jebzd9utIdLl0LMk","2025-05-22 12:33:37",0),
("limassu9731@gmail.com","0783232757","$argon2id$v=19$m=19456,t=2,p=1$7QjMsFwpuxcNVJFRy8f6qw$Lp6I1NRqskqDtYg8lVoMr56vO3vFuoYkXfbkZrNzTJA","2025-05-22 12:34:41",1);