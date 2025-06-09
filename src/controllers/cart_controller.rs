use actix_session::Session;
use actix_web::{HttpResponse, web};
use chrono::{DateTime, Duration, NaiveDateTime, Timelike, Utc};
use serde::Deserialize;

use crate::{
    models::{complex_request::get_cart_items, order_model::Order, user_model::User},
    routes::{ROUTE_CART_ORDER, ROUTE_CONTEXT},
    statics::TERA,
    utilities::{DynResult, render_to_response},
};

pub async fn index(session: Session) -> DynResult<HttpResponse> {
    let user = User::from_session_infallible(&session)?;
    let is_admin = user.is_admin();
    if is_admin {
        return Ok(HttpResponse::BadRequest().body("le panier est pour les utilisateurs"));
    }
    index_user(user)
}
fn index_user(user: User) -> DynResult<HttpResponse> {
    let mut context = ROUTE_CONTEXT.clone();
    let cart_id: i32 = user.cart_id()?;
    println!("cart_id: {cart_id}");
    let cart_items = get_cart_items(cart_id)?;
    context.insert("id_order", &cart_id);
    context.insert("items", &cart_items);
    context.insert("is_admin", &user.is_admin());
    context.insert("is_connected", &true);
    Ok(render_to_response(
        TERA.render("views/cart-user.html", &context),
    ))
}

#[derive(Deserialize)]
pub struct FormOrder {
    pub datetime: String,
}

pub async fn order_post(user: User, form: web::Form<FormOrder>) -> DynResult<HttpResponse> {
    let cart_id = user.cart_id()?;
    let datetime_str = &form.datetime;
    let cart_items = get_cart_items(cart_id)?;
    if cart_items.is_empty() {
        return Ok(HttpResponse::BadRequest().body("Cart is empty"));
    }
    let min_date_time = compute_min_datetime();
    let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%dT%H:%M")?;
    if datetime < min_date_time.naive_local() {
        return Ok(HttpResponse::BadRequest().body("Date invalide ou trop proche"));
    }
    let now = Utc::now();
    Order::order(
        cart_id,
        datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
        now.format("%Y-%m-%d %H:%M:%S").to_string(),
    )?;

    Ok(HttpResponse::Ok().body("Votre command à bien été prise en compte"))
}
pub async fn order_get(user: User) -> DynResult<HttpResponse> {
    let cart_id = user.cart_id()?;
    let cart_items = get_cart_items(cart_id)?;
    let total_price = cart_items
        .iter()
        .map(|p| p.price * p.quantity as f64)
        .sum::<f64>();
    let total_price_str = format!("{:.2}", total_price);
    let mut context = ROUTE_CONTEXT.clone();
    context.insert("items", &cart_items);
    context.insert("total_price", &total_price_str);
    context.insert(
        "min_time",
        &compute_min_datetime().format("%Y-%m-%dT%H:%M").to_string(),
    );

    Ok(render_to_response(
        TERA.render(ROUTE_CART_ORDER.file_path, &context),
    ))
}

pub async fn show() {}

pub async fn update() {}

pub async fn delete() {}

fn compute_min_datetime() -> DateTime<Utc> {
    let now = Utc::now();
    let hour = now.hour();

    if hour < 18 {
        // juste +24h
        now + Duration::hours(24)
    } else {
        // +24h + (temps jusqu'à 9h demain)
        // temps jusqu'à 9h demain = (24 - heure actuelle) + 9
        let hours_until_9am = (24 - hour) + 9;
        now + Duration::hours(24 + hours_until_9am as i64)
    }
}
