use crate::schema::category_product;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "category_product"]
pub struct CategoryProduct {
    pub id_products: i32,
    pub id_categories: i32,
}
