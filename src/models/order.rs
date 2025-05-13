use crate::schema::orders;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Insertable, Deserialize)]
#[table_name = "orders"]
pub struct Order {
    pub id_orders: i32,
    pub date_order: String, // YYYY-MM-DD HH:MM:SS
    pub date_retrieve: String,
    pub id_users: i32,
}
