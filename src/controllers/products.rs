use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Serialize;
use tera::Context;

use crate::{
    TERA,
    models::product::Product,
    routes::{ROUTE_CONTEXT, ROUTE_EDIT_PRODUCT, ROUTE_PRODUCTS},
    utilities::{self, Renderable, Responseable, new_internal_error, render_to_response},
};
#[derive(Serialize, Default)]
pub struct Products {
    products: Vec<Product>,
    is_admin: bool,
}

impl Renderable for Products {
    fn render(&self) -> Result<String, tera::Error> {
        let mut context = ROUTE_CONTEXT.clone();
        context.insert("products", &self.products);
        context.insert("is_admin", &self.is_admin);
        TERA.render(ROUTE_PRODUCTS.file_path, &context)
    }
}
impl Responseable for Products {}

pub async fn products_get(session: Session) -> impl Responder {
    let Some(products) = Product::all() else {
        return new_internal_error();
    };

    let products_view = Products {
        products: products,
        is_admin: utilities::is_admin(&session),
    };

    products_view.into_response()
}

pub async fn product_id_get(path: web::Path<i32>) -> impl Responder {
    println!("product_id_get called");
    let mut context = Context::new();
    context.extend(ROUTE_CONTEXT.clone());
    let product = if let Some(p) = Product::get(*path) {
        p
    } else {
        return HttpResponse::NotFound().body("No product exist with the given id");
    };
    context.insert("name", &product.name);
    context.insert("id_product", &product.id_product);
    context.insert("description", &product.description);
    context.insert("price", &product.price);
    context.insert("image_url", &product.image_url);

    render_to_response(TERA.render(ROUTE_EDIT_PRODUCT.file_path, &context))
}
