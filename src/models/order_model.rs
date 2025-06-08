use crate::schema::orders;
use crate::schema::orders::dsl::*;
use crate::statics::DB_POOL;
use crate::utilities::{DynResult, handle_optional_query_result};
use diesel::prelude::*;
use diesel::{RunQueryDsl, sql_query};
use serde::{Deserialize, Serialize};

use super::product_model::Product;

/// Représente un produit avec sa quantité dans une commande
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderProduct {
    pub quantity: i32,
    pub product: Product,
}

/// Représente les données d'une commande pour l'affichage
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDisplay {
    pub id_order: i32,
    pub date_order: String, // Format: YYYY-MM-DD HH:MM:SS
    pub date_retrieve: String,
    pub id_user: i32,
    pub order_state: i32,
}

/// Représente une commande avec ses produits pour l'affichage
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderWithProducts {
    pub order: OrderDisplay,
    pub products: Vec<OrderProduct>,
    pub total: f64,
}

/// Structure pour regrouper les commandes par statut
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OrdersByStatus {
    pub need_confirmation: Vec<OrderWithProducts>,
    pub confirmed: Vec<OrderWithProducts>,
    pub ready: Vec<OrderWithProducts>,
    pub purchased: Vec<OrderWithProducts>,
}

#[derive(Queryable, Serialize, Insertable, Deserialize, Debug)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id_order: i32,
    pub date_order: Option<String>, // YYYY-MM-DD HH:MM:SS
    pub date_retrieve: Option<String>,
    pub order_obj: Option<String>,
    pub id_user: i32,
    pub order_state: i32,
}
#[derive(Debug, Clone, Copy)]
pub enum OrderState {
    Cart,
    NeedConfirmation,
    Confirmed,
    Ready,
    Purnchased,
    Refused,
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
        if relateds.is_empty() {
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
impl Order {
    /// Récupère toutes les commandes non finalisées organisées par statut
    pub fn get_unfinished_orders() -> DynResult<OrdersByStatus> {
        use crate::schema::orders::dsl::*;

        let mut conn = DB_POOL.get()?;

        // Récupérer toutes les commandes non finalisées (tous les états sauf le panier)
        let unfinished_orders = orders
            .filter(order_state.ne(OrderState::Cart as i32))
            .load::<Order>(&mut conn)?;

        Self::classify_orders(unfinished_orders)
    }

    /// Récupère toutes les commandes d'un utilisateur spécifique, organisées par statut
    pub fn get_orders_by_user(user_id: i32) -> DynResult<OrdersByStatus> {
        use crate::schema::orders::dsl::*;

        let mut conn = DB_POOL.get()?;

        // Récupérer toutes les commandes de l'utilisateur (sauf le panier actif)
        let user_orders = orders
            .filter(id_user.eq(user_id))
            .filter(order_state.ne(OrderState::Cart as i32))
            .load::<Order>(&mut conn)?;

        Self::classify_orders(user_orders)
    }

    /// Convertit une commande en OrderWithProducts
    fn to_order_with_products(order: Order) -> Option<OrderWithProducts> {
        // Utilisation de noms différents pour éviter les conflits
        let order_id = order.id_order;
        let order_date = order.date_order?;
        let retrieve_date = order.date_retrieve?;
        let user_id = order.id_user;
        let state = order.order_state;

        let products: Vec<OrderProduct> = if let Some(obj) = order.order_obj {
            let json_obj: Result<JsonObj, _> = serde_json::from_str(&obj);
            match json_obj {
                Ok(json) => json
                    .data
                    .into_iter()
                    .map(|(qty, product)| OrderProduct {
                        quantity: qty,
                        product,
                    })
                    .collect(),
                Err(_) => return None,
            }
        } else {
            return None;
        };

        let order_display = OrderDisplay {
            id_order: order_id,
            date_order: order_date,
            date_retrieve: retrieve_date,
            id_user: user_id,
            order_state: state,
        };
        let total = products
            .iter()
            .map(|product| product.quantity as f64 * product.product.price)
            .sum();

        Some(OrderWithProducts {
            order: order_display,
            products,
            total,
        })
    }

    /// Classe les commandes par statut
    fn classify_orders(other_orders: Vec<Order>) -> DynResult<OrdersByStatus> {
        let mut result = OrdersByStatus::default();

        for order in other_orders {
            let Some(order_with_products) = Self::to_order_with_products(order) else {
                continue;
            };

            match order_with_products.order.order_state {
                s if s == OrderState::NeedConfirmation as i32 => {
                    result.need_confirmation.push(order_with_products);
                }
                s if s == OrderState::Confirmed as i32 => {
                    result.confirmed.push(order_with_products);
                }
                s if s == OrderState::Ready as i32 => {
                    result.ready.push(order_with_products);
                }
                s if s == OrderState::Purnchased as i32 => {
                    result.purchased.push(order_with_products);
                }
                _ => continue,
            }
        }

        Ok(result)
    }
    pub fn update_state(order_id: i32, state: OrderState) -> DynResult<bool> {
        let mut conn = DB_POOL.get()?;
        let updated = diesel::update(orders.filter(id_order.eq(order_id)))
            .set(order_state.eq(state as i32))
            .execute(&mut conn)?;

        Ok(updated > 0)
    }
}

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
    fn into_serialise(self) -> Result<std::string::String, serde_json::Error> {
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
