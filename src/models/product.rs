use crate::DB_POOL;
use crate::schema::products;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::products::dsl::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::query_dsl::methods::*;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct Product {
    pub id_product: i32,
    pub description: String,
    pub name: String,
    pub price: f64,
    pub image_url: String,
}
impl Product {
    pub fn get(id: i32) -> Option<Self> {
        let mut conn = DB_POOL.get().ok()?;
        products.find(id).first(&mut conn).ok()
    }
    pub fn all() -> Option<Vec<Self>> {
        let mut conn = DB_POOL.get().ok()?;
        products.load(&mut conn).ok()
    }
}
