use std::error::Error;

use actix_session::Session;
use actix_web::HttpResponse;

use crate::{
    models::{order_model::Order, user_model::User},
    routes::{ROUTE_CONTEXT, ROUTE_DASHBOARD},
    statics::TERA,
    utilities::{DynResult, add_login_propetry_to_context, render_to_response},
};

pub async fn dashboard_get(session: Session) -> DynResult<HttpResponse> {
    println!("handle connexion\nsession:{:?}", session.entries());
    let user = User::from_session_infallible(&session)?;
    let orders = if user.is_admin() {
        Order::get_unfinished_orders()?
    } else {
        Order::get_orders_by_user(user.id_user)?
    };

    // Créer le contexte avec les commandes
    let mut context = ROUTE_CONTEXT.clone();
    context.insert("orders", &orders);

    // Log de débogage
    println!("Orders structure: {:#?}", orders);

    add_login_propetry_to_context(&mut context, &session)?;

    // Log du contexte complet
    println!("Template context: {:#?}", context);

    let render_result = TERA.render(ROUTE_DASHBOARD.file_path, &context);

    match &render_result {
        Ok(_) => {}
        Err(e) => {
            println!("Template rendering error: {}", e);
            println!("Template source: {}", e.source().unwrap_or(e));
        }
    }

    Ok(render_to_response(render_result))
}
