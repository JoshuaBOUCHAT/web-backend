use actix_web::Responder;
use derive_builder::Builder;
use serde::Serialize;

use crate::{
    routes::{ROUTE_CONTEXT, ROUTE_WELCOME},
    statics::TERA,
    utilities::{Renderable, Responseable},
};

#[derive(Builder, Serialize)]
pub struct Welcome {}
impl Renderable for Welcome {
    fn render(&self) -> Result<String, tera::Error> {
        let context = ROUTE_CONTEXT.clone();
        TERA.render(ROUTE_WELCOME.file_path, &context)
    }
}
impl Responseable for Welcome {}

pub async fn welcome_get() -> impl Responder {
    Welcome {}.into_response()
}
