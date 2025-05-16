-- Step 1: Insert main categories (parents)
INSERT INTO categories (name, description, super_category)
VALUES 
  ('Viennoiseries', 'Baked goods typically eaten for breakfast', NULL),
  ('Patisseries', 'Sweet baked desserts', NULL);

-- Step 2: Insert subcategories using subqueries to fetch parent IDs

-- Viennoiseries subcategories
INSERT INTO categories (name, description, super_category)
VALUES 
  ('Pain au chocolat', 'Chocolate-filled puff pastry', (SELECT id_category FROM categories WHERE name = 'Viennoiseries')),
  ('Croissant', 'Flaky buttery pastry', (SELECT id_category FROM categories WHERE name = 'Viennoiseries')),
  ('Pain au raisin', 'Spiral pastry with raisins and custard', (SELECT id_category FROM categories WHERE name = 'Viennoiseries'));

-- Patisseries subcategories
INSERT INTO categories (name, description, super_category)
VALUES 
  ('Macaron', 'French almond meringue cookie with ganache', (SELECT id_category FROM categories WHERE name = 'Patisseries')),
  ('Tarte au citron', 'Lemon tart with sweet pastry crust', (SELECT id_category FROM categories WHERE name = 'Patisseries')),
  ('Éclair', 'Choux pastry filled with cream and topped with icing', (SELECT id_category FROM categories WHERE name = 'Patisseries'));
