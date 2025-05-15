use crate::schema::order_product;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Insertable, Deserialize)]
#[diesel(table_name = order_product)]
pub struct OrderProduct {
    pub id_order: i32,
    pub id_product: i32,
    pub quantity: i32,
}
