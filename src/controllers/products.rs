use actix_web::Responder;
use serde::Serialize;

use crate::{
    TERA,
    models::product::Product,
    routes::{ROUTE_CONTEXT, ROUTE_PRODUCTS},
    traits::{Renderable, Responseable},
};
#[derive(Serialize, Default)]
pub struct Products {
    products: Vec<Product>,
}
impl Renderable for Products {
    fn render(&self) -> Result<String, tera::Error> {
        let mut context = ROUTE_CONTEXT.clone();
        TERA.render(ROUTE_PRODUCTS.file_path, &context)
    }
}
impl Responseable for Products {}

pub async fn products_get() -> impl Responder {
    Products::default().into_response()
}
