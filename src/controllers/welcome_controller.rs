use actix_web::HttpResponse;

use crate::{
    routes::{ROUTE_CONTEXT, ROUTE_WELCOME},
    statics::TERA,
    utilities::{DynResult, add_login_propetry_to_context, render_to_response},
};

pub async fn welcome_get(session: actix_session::Session) -> DynResult<HttpResponse> {
    let mut context = ROUTE_CONTEXT.clone();
    add_login_propetry_to_context(&mut context, &session)?;
    Ok(render_to_response(
        TERA.render(ROUTE_WELCOME.file_path, &context),
    ))
}
