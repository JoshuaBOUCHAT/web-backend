use crate::schema::orders;
use crate::schema::orders::dsl::*;
use crate::statics::DB_POOL;
use diesel::RunQueryDsl;
use diesel::prelude::{Insertable, Queryable};
use diesel::query_dsl::methods::FindDsl;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Insertable, Deserialize)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id_order: i32,
    pub date_order: String, // YYYY-MM-DD HH:MM:SS
    pub date_retrieve: String,
    pub id_user: i32,
}
impl Order {
    pub fn get(id: i32) -> Option<Self> {
        let mut conn = DB_POOL.get().ok()?;
        orders.find(id).first(&mut conn).ok()
    }
    pub fn all() -> Option<Vec<Self>> {
        let mut conn = DB_POOL.get().ok()?;
        orders.load(&mut conn).ok()
    }
}
