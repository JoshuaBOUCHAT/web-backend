use crate::schema::products;
use crate::statics::DB_POOL;
use crate::utilities::{DynResult, get_db, handle_optional_query_result};
use derive_builder::Builder;
use diesel::prelude::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::products::dsl::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::dsl::{delete, insert_into};
use diesel::query_dsl::methods::*;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = products)]
pub struct Product {
    pub id_product: i32,
    pub description: String,
    pub name: String,
    pub price: f64,
    pub image_url: String,
    pub visible: i32,
}

#[derive(AsChangeset, Deserialize, Debug, Builder)]
#[diesel(table_name = products)]
pub struct ProductPatch {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub image_url: Option<String>,
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub description: String,
    pub name: String,
    pub price: f64,
    pub image_url: String,
}
impl Product {
    pub fn get(id: i32) -> DynResult<Option<Self>> {
        let mut conn = DB_POOL.get()?;
        handle_optional_query_result(
            products.find(id).first::<Self>(&mut conn),
            format!("Error happen while trying to get product with id {id} err:\n"),
        )
    }
    pub fn all() -> DynResult<Vec<Self>> {
        let mut conn = DB_POOL.get()?;
        Ok(products.load(&mut conn)?)
    }
    pub fn delete(id: i32) -> bool {
        get_db().is_some_and(|mut conn| {
            delete(products.filter(id_product.eq(id)))
                .execute(&mut conn)
                .map(|rows_affected| rows_affected > 0)
                .unwrap_or(false)
        })
    }
    pub fn patch(id: i32, update_data: ProductPatch) -> DynResult<Option<Self>> {
        let mut conn = DB_POOL.get()?;

        let affected_rows = diesel::update(products.filter(id_product.eq(id)))
            .set(update_data)
            .execute(&mut conn)?;

        if affected_rows == 0 {
            Ok(None)
        } else {
            Self::get(id) // Return updated product
        }
    }
    pub fn create(new_product: NewProduct) -> DynResult<Self> {
        let mut conn = DB_POOL.get()?;

        insert_into(products)
            .values(&new_product)
            .execute(&mut conn)?;

        // Assuming `id_product` is auto-increment and you want to fetch the last inserted row,
        // you may need to fetch it again by some unique field or return the inserted data directly.
        // Here, let's assume `new_product` has a unique name and we fetch by that:

        products
            .filter(name.eq(&new_product.name))
            .order(id_product.desc()) // in case multiple with same name, get the latest
            .first::<Self>(&mut conn)
            .map_err(|e| e.into())
    }
    pub fn update_visibility(id: i32, visibility_value: i32) -> DynResult<bool> {
        let mut conn = DB_POOL.get()?;
        let updated = diesel::update(products.filter(id_product.eq(id)))
            .set(visible.eq(visibility_value))
            .execute(&mut conn)?;

        Ok(updated > 0)
    }
}
