use actix_session::Session;
use actix_web::{HttpResponse, web};

use crate::{
    models::{order_product_model::OrderProduct, user_model::User},
    try_or_return,
    utilities::ExtractHttp,
};

pub async fn update(path: web::Path<(i32, i32)>, session: Session) -> HttpResponse {
    eprintln!("test");
    let (product_id, qty) = *(path);
    eprintln!("get the qty and id product");
    let user = try_or_return!(User::from_session_infallible(&session).extract_http());
    eprintln!("get the user");
    let order_id = try_or_return!(user.cart_id().extract_http());
    eprintln!("get the order id");
    try_or_return!(OrderProduct::update(order_id, product_id, qty).extract_http());
    eprintln!("update the cart");

    if qty == 1 {
        HttpResponse::Ok().body("Le  produit à bien été ajouté")
    } else {
        HttpResponse::Ok().body("Les produits on bien été ajouté")
    }
}
