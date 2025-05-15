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
    pub id_products: i32,
    pub description: String,
    pub name: String,
    pub price: f64,
    pub image_url: Option<String>,
}
impl Product {
    pub fn all() -> Vec<Product> {
        let mut db = DB_POOL
            .get()
            .expect("getting DB_POOL failed in products.rs");
        products
            .load(&mut db)
            .expect("querry all in products failed")
    }
}
