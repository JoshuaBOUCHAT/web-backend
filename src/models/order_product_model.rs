use super::{order_model::Order, product_model::Product};
use crate::{
    schema::{
        order_product::{self, dsl::*},
        products,
    },
    statics::DB_POOL,
    utilities::{DynResult, handle_optional_query_result},
};
use diesel::JoinOnDsl;
use diesel::{
    BoolExpressionMethods, ExpressionMethods,
    prelude::{Insertable, Queryable},
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel::{RunQueryDsl, prelude::AsChangeset};
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = order_product)]
pub struct OrderProduct {
    pub id_order: i32,
    pub id_product: i32,
    pub quantity: i32,
}

impl OrderProduct {
    pub fn update(order_id: i32, product_id: i32, qty: i32) -> DynResult<()> {
        let mut conn = DB_POOL.get()?;
        let affected = diesel::update(
            order_product.filter(id_product.eq(product_id).and(id_order.eq(order_id))),
        )
        .set(OrderProduct {
            id_order: order_id,
            id_product: product_id,
            quantity: qty,
        })
        .execute(&mut conn)?;
        if affected == 0 {
            println!("not affected");
            diesel::insert_into(order_product)
                .values((
                    id_order.eq(order_id),
                    id_product.eq(product_id),
                    quantity.eq(qty),
                ))
                .execute(&mut conn)?;
        } else {
            println!("affected");
        }
        Ok(())
    }
    pub fn qty_from_cart_and_product(cart_id: i32, product_id: i32) -> DynResult<Option<i32>> {
        use crate::schema::order_product::dsl as op;
        let mut conn = DB_POOL.get()?;
        let querry = op::order_product
            .filter(op::id_order.eq(cart_id).and(op::id_product.eq(product_id)))
            .select(op::quantity)
            .first(&mut conn);
        let opt: Option<i32> = handle_optional_query_result(
            querry,
            "Error when trying to et quantity of a product in a cart",
        )?;
        Ok(opt)
    }
    pub fn delete(order_id: i32, product_id: i32) -> DynResult<bool> {
        let mut conn = DB_POOL.get()?;
        let affected = diesel::delete(
            order_product.filter(id_order.eq(order_id).and(id_product.eq(product_id))),
        )
        .execute(&mut conn)?;
        Ok(affected != 0)
    }
}
