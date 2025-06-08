use crate::{
    routes::{ROUTE_CONTEXT, STATIC_ROUTES},
    statics::TERA,
    utilities::{self, DynResult, render_to_response},
};
use actix_session::SessionExt;
use actix_web::{HttpRequest, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct Welcome {
    name: String,
}
///handle /static/{}
pub async fn static_route_get(req: HttpRequest) -> DynResult<HttpResponse> {
    println!("passed here!");
    let route = req.path();
    if let Some(r) = STATIC_ROUTES.iter().find(|&r| r.web_path == route) {
        let session = req.get_session();
        let mut context = ROUTE_CONTEXT.clone();
        utilities::add_login_propetry_to_context(&mut context, &session)?;
        Ok(render_to_response(TERA.render(r.file_path, &context)))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
