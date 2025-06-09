use actix_web::HttpResponse;

use crate::{
    models::{order_model::Order, user_model::User},
    routes::{ROUTE_CONTEXT, ROUTE_DASHBOARD},
    statics::TERA,
    utilities::{DynResult, render_to_response},
};

pub async fn dashboard_get(user: User) -> DynResult<HttpResponse> {
    let orders = if user.is_admin() {
        Order::get_unfinished_orders()?
    } else {
        Order::get_orders_by_user(user.id_user)?
    };

    // Créer le contexte avec les commandes
    let mut context = ROUTE_CONTEXT.clone();
    context.insert("orders", &orders);

    context.extend(user.get_login_context());

    Ok(render_to_response(
        TERA.render(ROUTE_DASHBOARD.file_path, &context),
    ))
}
