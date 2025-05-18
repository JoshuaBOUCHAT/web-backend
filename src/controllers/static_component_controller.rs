use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use crate::{
    routes::{ROUTE_CONTEXT, STATIC_ROUTES},
    statics::TERA,
    utilities::render_to_response,
};

#[derive(Serialize)]
pub struct Welcome {
    name: String,
}
///handle /static/{}
pub async fn static_route_get(req: HttpRequest) -> impl Responder {
    println!("passed here!");
    let route = req.path();
    if let Some(r) = STATIC_ROUTES.iter().find(|&r| r.web_path == route) {
        render_to_response(TERA.render(r.file_path, &ROUTE_CONTEXT))
    } else {
        HttpResponse::NotFound().finish()
    }
}
