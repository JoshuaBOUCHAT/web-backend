use diesel::RunQueryDsl;
use diesel::prelude::QueryableByName;
use diesel::sql_query;
use diesel::sql_types::{Double, Integer, Nullable, Text};
use serde::{Deserialize, Serialize};

use crate::schema::{order_product, products};
use crate::statics::DB_POOL;
use crate::utilities::DynResult;

#[derive(QueryableByName, Debug, Serialize)]
pub struct ProductWithCategories {
    #[diesel(sql_type = Integer)]
    pub id_product: i32,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub description: String,
    #[diesel(sql_type = Double)]
    pub price: f64,
    #[diesel(sql_type = Text)]
    pub image_url: String,
    #[diesel(sql_type = Integer)]
    pub visible: i32,
    #[diesel(sql_type = Nullable<Text>)]
    pub categories_ids: Option<String>, // Liste concaténée sous forme de chaîne "1,3,5"
}
pub fn load_products_with_categories() -> DynResult<Vec<ProductWithCategories>> {
    let mut conn = DB_POOL.get()?;
    let query = "
        SELECT 
            p.id_product, 
            p.name, 
            p.description, 
            p.price, 
            p.image_url,
            p.visible,
            GROUP_CONCAT(cp.id_category) AS categories_ids
        FROM products p
        LEFT JOIN category_product cp ON p.id_product = cp.id_product
        GROUP BY p.id_product;
    ";

    Ok(sql_query(query).load::<ProductWithCategories>(&mut conn)?)
}

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct CartItem {
    pub id_product: i32,
    pub name: String,
    pub price: f64,
    pub image_url: String,
    pub quantity: i32,
    pub description: String,
}

use diesel::prelude::*;
pub fn get_cart_items(id_order: i32) -> DynResult<Vec<CartItem>> {
    let mut conn = DB_POOL.get()?;

    let results = order_product::table
        .inner_join(products::table.on(products::id_product.eq(order_product::id_product)))
        .filter(order_product::id_order.eq(id_order))
        .select((
            products::id_product,
            products::name,
            products::price,
            products::image_url,
            order_product::quantity,
            products::description,
        ))
        .load::<CartItem>(&mut conn)?;

    Ok(results)
}
