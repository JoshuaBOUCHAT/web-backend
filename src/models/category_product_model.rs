use crate::schema::category_product;
use crate::{schema::category_product::dsl::*, statics::DB_POOL, utilities::DynResult};
use diesel::RunQueryDsl;
use diesel::prelude::{Insertable, Queryable};
use diesel::query_dsl::methods::*;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = category_product)]
pub struct CategoryProduct {
    pub id_product: i32,
    pub id_category: i32,
}
impl CategoryProduct {
    pub fn all() -> DynResult<Vec<Self>> {
        let mut conn = DB_POOL.get()?;
        Ok(category_product.load(&mut conn)?)
    }
}
