use crate::schema::order_product;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Insertable, Deserialize)]
#[table_name = "order_product"]
pub struct OrderProduct {
    pub id_orders: i32,
    pub id_products: i32,
    pub nombre: i32,
}
