use std::{collections::HashMap, sync::LazyLock};

use actix_web::web::*;
use tera::Context;

use crate::controllers::{
    auth_controller, cart_controller, category_controller, dashboard_controller, order_controller,
    products_controller, static_component_controller, verify_controller, welcome_controller,
};

pub struct Route {
    pub web_path: &'static str,
    pub file_path: &'static str,
}
impl Route {
    const fn new(web_path: &'static str, file_path: &'static str) -> Self {
        Self {
            web_path,
            file_path,
        }
    }
}

pub const ROUTE_PUBLIC: Route = Route::new("/public", "public");

pub const ROUTE_EDIT_PRODUCT: Route =
    Route::new("/products/{id_product}", "partials/edit_product.html");
pub const ROUTE_DELETE_PRODUCT: &'static str = "/products/{id}";
pub const ROUTE_PRODUCTS: Route = Route::new("/products", "views/products.html");
pub const ROUTE_PRODUCT_NEW: &'static str = "/products";
pub const ROUTE_PRODUCT_VISIBILITY: &'static str = "/product/{id}/visibility";

pub const ROUTE_CATEGORY_NEW: Route = Route::new("/category/new", "partials/create-category.html");
pub const ROUTE_CATEGORY_EDIT: Route =
    Route::new("/category/edit/{id}", "partials/edit-category.html");
pub const ROUTE_CATEGORY_SELECT: Route =
    Route::new("/category/select", "partials/select-category.html");

pub const ROUTE_CATEGORY_DELETE: &'static str = "/category/{id}";

pub const ROUTE_WELCOME: Route = Route::new("/", "views/welcome.html");

pub const ROUTE_DASHBOARD: Route = Route::new("/dashboard", "views/dashboard.html");
pub const ROUTE_CART: Route = Route::new("/cart", "/views/cart.html");
pub const ROUTE_CART_ORDER: Route = Route::new("/cart/order", "partials/cart/order-cart.html");

pub const ROUTE_ORDER: Route = Route::new("/order/{id}", "partials/add-cart-menu.html");
pub const ROUTE_ORDER_QTY: &'static str = "/order/{id}/{qty}";

pub const ROUTE_REGISTER: &'static str = "/register";
pub const ROUTE_LOGIN: &'static str = "/login";
pub const ROUTE_LOGOUT: &'static str = "/logout";

pub const ROUTE_FOOTER: Route = Route::new("/static/footer", "static/footer.html");
pub const ROUTE_NAV: Route = Route::new("/static/footer", "nav/footer.html");
pub const ROUTE_ABOUT: Route = Route::new("/static/about", "static/about.html");
pub const ROUTE_AUTH: Route = Route::new("/auth", "views/auth.html");
pub const ROUTE_VERIFY: Route = Route::new("/auth/verify", "views/verify.html");
pub const ROUTE_ORDER_STATE: &'static str = "/orders/{id}/state/{state}";

pub const ROUTE_STATICS: &'static str = "/static/{route}";

pub const STATIC_ROUTES: [Route; 4] = [ROUTE_FOOTER, ROUTE_NAV, ROUTE_ABOUT, ROUTE_AUTH];

fn get_route_context() -> Context {
    let mut routes: HashMap<&str, &str> = HashMap::new();

    routes.insert("welcome", ROUTE_WELCOME.web_path);
    routes.insert("products", ROUTE_PRODUCTS.web_path);
    routes.insert("auth", ROUTE_AUTH.web_path);
    routes.insert("dashboard", ROUTE_DASHBOARD.web_path);
    routes.insert("register", ROUTE_REGISTER);
    routes.insert("login", ROUTE_LOGIN);
    routes.insert("logout", ROUTE_LOGOUT);
    routes.insert("footer", ROUTE_FOOTER.web_path);
    routes.insert("nav", ROUTE_NAV.web_path);
    routes.insert("about", ROUTE_ABOUT.web_path);
    routes.insert("edit_product", ROUTE_EDIT_PRODUCT.web_path);
    routes.insert("cart", ROUTE_CART.web_path);

    let mut context = Context::new();
    context.insert("routes", &routes);

    context
}
pub static ROUTE_CONTEXT: LazyLock<Context> = LazyLock::new(|| get_route_context());

pub fn configure_guess_routes(cfg: &mut actix_web::web::ServiceConfig) {
    use actix_web::web::*;

    cfg.route(ROUTE_REGISTER, post().to(auth_controller::register_post))
        .route(ROUTE_LOGIN, post().to(auth_controller::login_post))
        .route(
            ROUTE_STATICS,
            get().to(static_component_controller::static_route_get),
        )
        .route(
            ROUTE_WELCOME.web_path,
            get().to(welcome_controller::welcome_get),
        )
        .route(
            ROUTE_PRODUCTS.web_path,
            get().to(products_controller::products_get),
        )
        .route(ROUTE_AUTH.web_path, get().to(auth_controller::auth_get))
        .route(ROUTE_VERIFY.web_path, get().to(verify_controller::index));
}
pub fn configure_auth_routes(cfg: &mut actix_web::web::ServiceConfig) {
    use actix_web::web::*;
    cfg.route(ROUTE_CART.web_path, get().to(cart_controller::index))
        .route(ROUTE_LOGOUT, get().to(auth_controller::logout_get))
        .route(ROUTE_ORDER_QTY, put().to(order_controller::update))
        .route(ROUTE_ORDER.web_path, delete().to(order_controller::destroy))
        .route(
            ROUTE_CART_ORDER.web_path,
            get().to(cart_controller::order_get),
        )
        .route(
            ROUTE_CART_ORDER.web_path,
            post().to(cart_controller::order_post),
        );
}

pub fn configure_admin_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route(
        ROUTE_DASHBOARD.web_path,
        get().to(dashboard_controller::dashboard_get),
    )
    .route(
        ROUTE_EDIT_PRODUCT.web_path,
        get().to(products_controller::product_id_get),
    )
    .route(
        ROUTE_DELETE_PRODUCT,
        delete().to(products_controller::product_id_delete),
    )
    .route(
        ROUTE_EDIT_PRODUCT.web_path,
        patch().to(products_controller::product_id_patch),
    )
    .route(
        ROUTE_PRODUCT_NEW,
        post().to(products_controller::product_post),
    )
    .route(
        ROUTE_PRODUCT_VISIBILITY,
        put().to(products_controller::product_put_visibility),
    )
    .route(
        ROUTE_CATEGORY_NEW.web_path,
        post().to(category_controller::new_post),
    )
    .route(
        ROUTE_CATEGORY_NEW.web_path,
        get().to(category_controller::new_get),
    )
    .route(
        ROUTE_CATEGORY_SELECT.web_path,
        get().to(category_controller::select_get),
    )
    .route(
        ROUTE_CATEGORY_EDIT.web_path,
        get().to(category_controller::edit_get),
    )
    .route(
        ROUTE_CATEGORY_DELETE,
        delete().to(category_controller::destroy),
    )
    .route(
        ROUTE_CATEGORY_EDIT.web_path,
        patch().to(category_controller::edit_post),
    )
    .route(ROUTE_ORDER.web_path, get().to(order_controller::edit))
    .route(ROUTE_ORDER_STATE, post().to(order_controller::update_state));
}
