use std::{collections::HashMap, sync::LazyLock};

use tera::Context;

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
pub const ROUTE_PRODUCT_NEW: &'static str = "/product";
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
pub const ROUTE_ORDER: &'static str = "/order/{id}/{qty}";

pub const ROUTE_REGISTER: &'static str = "/register";
pub const ROUTE_LOGIN: &'static str = "/login";
pub const ROUTE_LOGOUT: &'static str = "/logout";

pub const ROUTE_FOOTER: Route = Route::new("/static/footer", "static/footer.html");
pub const ROUTE_NAV: Route = Route::new("/static/footer", "nav/footer.html");
pub const ROUTE_ABOUT: Route = Route::new("/static/about", "static/about.html");
pub const ROUTE_AUTH: Route = Route::new("/auth", "views/auth.html");

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

/*pub const WEB_ROUTES: [&'static str; 13] = [
    ROUTE_WELCOME.web_path,
    ROUTE_PRODUCTS.web_path,
    ROUTE_CSS.web_path,
    ROUTE_JS.web_path,
    ROUTE_IMAGES.web_path,
    ROUTE_AUTH.web_path,
    ROUTE_DASHBOARD.web_path,
    ROUTE_REGISTER,
    ROUTE_LOGIN,
    ROUTE_LOGOUT,
    ROUTE_FOOTER.web_path,
    ROUTE_NAV.web_path,
    ROUTE_ABOUT.web_path,
];*/
