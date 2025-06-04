use crate::schema::orders;
use crate::schema::orders::dsl::*;
use crate::statics::DB_POOL;
use crate::utilities::{DynResult, handle_optional_query_result};
use diesel::prelude::*;
use diesel::{RunQueryDsl, sql_query};
use serde::{Deserialize, Serialize};

use super::product_model::Product;
#[derive(Queryable, Serialize, Insertable, Deserialize)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id_order: i32,
    pub date_order: Option<String>, // YYYY-MM-DD HH:MM:SS
    pub date_retrieve: Option<String>,
    pub order_obj: Option<String>,
    pub id_user: i32,
    pub order_state: i32,
}
enum OrderState {
    Cart,
    NeedConfirmation,
    Confirmed,
    Ready,
    Purnchased,
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

    pub fn related_product(order_id: i32) -> DynResult<Vec<(i32, Product)>> {
        let mut conn = DB_POOL.get()?;

        let rows: Vec<Row> = sql_query(
            r#"
        SELECT 
            op.quantity,
            p.id_product,
            p.description,
            p.name,
            p.price,
            p.image_url,
            p.visible
        FROM order_product op
        INNER JOIN products p ON p.id_product = op.id_product
        WHERE op.id_order = ?
    "#,
        )
        .bind::<Integer, _>(order_id)
        .load(&mut conn)?;

        // Transformer les lignes SQL en tuple (i32, Product)
        let results = rows
            .into_iter()
            .map(|row| {
                let product = Product {
                    id_product: row.id_product,
                    description: row.description,
                    name: row.name,
                    price: row.price,
                    image_url: row.image_url,
                    visible: row.visible,
                };
                (row.quantity, product)
            })
            .collect();

        Ok(results)
    }
    //asume that the caller verify the dates
    pub fn order(cart_id: i32, retreive_date: String, order_date: String) -> DynResult<bool> {
        let mut conn = DB_POOL.get()?;
        let relateds = Order::related_product(cart_id)?;
        if relateds.len() == 0 {
            return Ok(false);
        }
        let json_obj = JsonObj { data: relateds }.into_serialise()?;
        let update_data = CartUpdate {
            date_order: order_date,
            date_retrieve: retreive_date,
            order_obj: json_obj,
            order_state: OrderState::NeedConfirmation as i32,
        };
        diesel::update(orders.filter(id_order.eq(cart_id)))
            .set(&update_data)
            .execute(&mut conn)?;

        Ok(true)
    }
}
impl Order {}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::orders)]
pub struct NewOrder {
    pub id_user: i32,
    pub date_order: Option<String>,
    pub date_retrieve: Option<String>,
}
use diesel::sql_types::{Double, Integer, Text};
#[derive(QueryableByName)]
pub struct Row {
    #[diesel(sql_type = Integer)]
    pub quantity: i32,

    #[diesel(sql_type = Integer)]
    pub id_product: i32,

    #[diesel(sql_type = Text)]
    pub description: String,

    #[diesel(sql_type = Text)]
    pub name: String,

    #[diesel(sql_type = Double)]
    pub price: f64,

    #[diesel(sql_type = Text)]
    pub image_url: String,

    #[diesel(sql_type = Integer)]
    pub visible: i32,
}

#[derive(Serialize, Deserialize)]
pub struct JsonObj {
    pub data: Vec<(i32, Product)>,
}
impl JsonObj {
    fn into_serialise(&self) -> Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::orders)]
pub struct CartUpdate {
    pub date_order: String,
    pub date_retrieve: String,
    pub order_obj: String,
    pub order_state: i32,
}
