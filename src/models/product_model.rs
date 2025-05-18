use crate::schema::products;
use crate::statics::DB_POOL;
use crate::utilities::get_db;
use diesel::dsl::sql;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::products::dsl::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::dsl::delete;
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
    pub fn delete(id: i32) -> bool {
        get_db().map_or(false, |mut conn| {
            delete(products.filter(id_product.eq(id)))
                .execute(&mut conn)
                .map(|rows_affected| rows_affected > 0)
                .unwrap_or(false)
        })
    }
}
