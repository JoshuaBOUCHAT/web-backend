use actix_session::Session;
use actix_web::{HttpResponse, Responder, web::Form};

use crate::{
    log,
    models::{complex_request::get_cart_items, user_model::User},
    routes::{ROUTE_CART, ROUTE_CONTEXT},
    statics::TERA,
    try_or_return,
    utilities::{ExtractHttp, render_to_response},
};

pub async fn index(session: Session) -> impl Responder {
    let user = try_or_return!(
        User::from_session_infallible(&session).extract_http(),
        "User try to index the cart without being logged in !"
    );
    let is_admin = user.is_admin();
    if is_admin {
        index_admin()
    } else {
        index_user(user)
    }
}
fn index_user(user: User) -> HttpResponse {
    let mut context = ROUTE_CONTEXT.clone();
    let order_id: i32 = try_or_return!(user.cart_id().extract_http());
    let cart_items = try_or_return!(get_cart_items(order_id).extract_http());
    context.insert("id_order", &order_id);
    context.insert("items", &cart_items);
    render_to_response(TERA.render("views/cart-user.html", &context))
}
fn index_admin() -> HttpResponse {
    todo!()
}

pub async fn show() {}

pub async fn update() {}

pub async fn delete() {}
