use derive_builder::Builder;
use serde::Serialize;
use tera::Context;

use crate::{
    TERA,
    traits::{Renderable, Responseable},
};

#[derive(Builder, Serialize)]
pub struct Welcome {
    name: String,
}
impl Renderable for Welcome {
    fn render(&self) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("welcome", self);
        TERA.render("welcome.html", &context)
    }
}
impl Responseable for Welcome {}
