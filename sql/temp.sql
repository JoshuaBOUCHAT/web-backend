SELECT
    GROUP_CONCAT(
        p.name,
        p.price,
        p.image_url,
        op.quantity)
FROM 
    users u
RIGHT JOIN 
    orders o
ON
    o.id_user = u.id_user
WHERE
    o.id_user IS NOT NULL AND
    o.date_retreive IS NULL AND 
    o.date_order IS NULL
RIGHT JOIN
    order_product op
ON
    op.id_order = o.id_order
WHERE
    op.id_order IS NOT NULL
FULL JOIN
    products p
ON
    p.id_product = op.id_product



SELECT
    o.id_order
FROM 
    users u
RIGHT JOIN 
    orders o
ON
    o.id_user = u.id_user
WHERE
    o.date_retreive IS NULL AND o.date_order IS NULL