use crate::schema::products;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "products"]
pub struct Product {
    pub id_products: i32,
    pub description: String,
    pub name: String,
    pub price: f64,
    pub image_url: String,
}
