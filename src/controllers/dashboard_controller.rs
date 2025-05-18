use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use derive_builder::Builder;
use serde::Serialize;
use tera::Context;

use crate::{
    routes::ROUTE_DASHBOARD,
    statics::TERA,
    utilities::{Renderable, Responseable},
};

#[derive(Builder, Serialize)]
pub struct Dashboard {
    id: i32,
}
impl Renderable for Dashboard {
    fn render(&self) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("dashboard", self);
        TERA.render(ROUTE_DASHBOARD.file_path, &context)
    }
}
impl Responseable for Dashboard {}

pub async fn dashboard_get(session: Session) -> impl Responder {
    println!("handle connexion\nsession:{:?}", session.entries());
    if let Some(id) = session.get::<i32>("user_id").unwrap() {
        println!("access granted");
        DashboardBuilder::default()
            .id(id)
            .build()
            .unwrap()
            .into_response()
    } else {
        println!("Need authentification");
        HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish()
    }
}
