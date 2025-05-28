use actix_session::Session;
use actix_web::{HttpResponse, Responder};

use crate::{
    models::{complex_request::get_cart_items, user_model::User},
    routes::ROUTE_CONTEXT,
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

pub async fn show() {}

pub async fn update() {}

pub async fn delete() {}
