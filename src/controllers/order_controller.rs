use actix_session::Session;
use actix_web::{
    HttpResponse,
    web::{self, Path},
};
use tera::Context;

use crate::{
    models::{order_product_model::OrderProduct, product_model::Product, user_model::User},
    routes::ROUTE_ORDER,
    statics::TERA,
    utilities::{DynResult, render_to_response},
};

pub async fn update(path: web::Path<(i32, i32)>, session: Session) -> DynResult<HttpResponse> {
    let (product_id, qty) = *(path);
    let user = User::from_session_infallible(&session)?;
    let order_id = user.cart_id()?;
    println!("cart_id:{order_id}");
    OrderProduct::update(order_id, product_id, qty)?;

    let message = if qty == 1 {
        "Le  produit à bien été ajouté"
    } else {
        "Les produits on bien été ajouté"
    };
    Ok(HttpResponse::Ok().body(message))
}
pub async fn edit(path: Path<i32>, session: Session) -> DynResult<HttpResponse> {
    let user = User::from_session_infallible(&session)?;
    let id_product = *path;

    let Some(product) = Product::get(id_product)? else {
        return Err("The product do not exist !")?;
    };

    let obj = OrderProduct::qty_from_cart_and_product(user.cart_id()?, product.id_product)?;
    let (already_in_cart, qty) = if let Some(qty) = obj {
        (true, qty)
    } else {
        (false, 1)
    };
    let mut context = Context::new();
    context.insert("qty", &qty);
    context.insert("already_in_cart", &already_in_cart);
    context.insert("id_product", &product.id_product);
    context.insert("name", &product.name);
    context.insert("description", &product.description);

    Ok(render_to_response(
        TERA.render(ROUTE_ORDER.file_path, &context),
    ))
}

pub async fn destroy(path: web::Path<i32>, session: Session) -> DynResult<HttpResponse> {
    let product_id = *path;
    let user = User::from_session_infallible(&session)?;
    let cart_id = user.cart_id()?;
    println!("cart_id: {cart_id}");
    let is_deleted = OrderProduct::delete(cart_id, product_id)?;

    let resp = if is_deleted {
        HttpResponse::Ok().body("Le produit à été suprimer du panier")
    } else {
        HttpResponse::BadRequest().body("Une erreur est survenu lors de la supression")
    };
    Ok(resp)
}
