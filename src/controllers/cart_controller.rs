use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use chrono::{DateTime, Duration, Timelike, Utc};

use crate::{
    models::{
        complex_request::{self, get_cart_items},
        user_model::User,
    },
    routes::{ROUTE_CART_ORDER, ROUTE_CONTEXT},
    statics::TERA,
    try_or_return,
    utilities::{DynResult, ExtractHttp, render_to_response},
};

pub async fn index(session: Session) -> DynResult<HttpResponse> {
    let user = User::from_session_infallible(&session)?;
    let is_admin = user.is_admin();
    if is_admin {
        index_admin()
    } else {
        index_user(user)
    }
}
fn index_user(user: User) -> DynResult<HttpResponse> {
    let mut context = ROUTE_CONTEXT.clone();
    let cart_id: i32 = user.cart_id()?;
    println!("cart_id: {cart_id}");
    let cart_items = get_cart_items(cart_id)?;
    context.insert("id_order", &cart_id);
    context.insert("items", &cart_items);
    Ok(render_to_response(
        TERA.render("views/cart-user.html", &context),
    ))
}
fn index_admin() -> DynResult<HttpResponse> {
    todo!()
}
pub async fn order_post(session: Session) -> DynResult<HttpResponse> {
    let user = User::from_session_infallible(&session)?;
    let cart_id = user.cart_id()?;
    //let

    todo!()
}
pub async fn order_get(session: Session) -> DynResult<HttpResponse> {
    let user = User::from_session_infallible(&session)?;
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
