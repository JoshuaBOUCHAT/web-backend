use crate::schema::orders;
use crate::schema::orders::dsl::*;
use crate::statics::DB_POOL;
use crate::utilities::{DynResult, handle_optional_query_result};
use diesel::RunQueryDsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Insertable, Deserialize)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id_order: i32,
    pub date_order: Option<String>, // YYYY-MM-DD HH:MM:SS
    pub date_retrieve: Option<String>,
    pub id_user: i32,
}
impl Order {
    pub fn get(id: i32) -> DynResult<Option<Self>> {
        let mut conn = DB_POOL.get()?;
        handle_optional_query_result(
            orders.find(id).first(&mut conn),
            format!("error when handling order get on id :{id}"),
        )
    }
    pub fn all() -> DynResult<Vec<Self>> {
        let mut conn = DB_POOL.get()?;
        Ok(orders.load(&mut conn)?)
    }

    pub fn create_order_for_user(user_id: i32) -> DynResult<i32> {
        let mut conn = DB_POOL.get()?;

        let new_order = NewOrder {
            id_user: user_id,
            date_order: None,
            date_retrieve: None,
        };

        diesel::insert_into(orders::table)
            .values(&new_order)
            .execute(&mut conn)?;
        let res: Order = orders.filter(id_order.eq(user_id)).first(&mut conn)?;

        Ok(res.id_order)
    }
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::orders)]
pub struct NewOrder {
    pub id_user: i32,
    pub date_order: Option<String>,
    pub date_retrieve: Option<String>,
}
