use std::{collections::HashMap, sync::LazyLock};

use actix_web::HttpResponse;
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

pub const ROUTE_CSS: Route = Route::new("/css", "public/css");
pub const ROUTE_JS: Route = Route::new("/img", "public/images");
pub const ROUTE_IMAGES: Route = Route::new("/js", "public/js");

pub const ROUTE_WELCOME: Route = Route::new("/", "views/welcome.html");
pub const ROUTE_PRODUCTS: Route = Route::new("/products", "views/products.html");
pub const ROUTE_DASHBOARD: Route = Route::new("/dashboard", "views/dashboard.html");

pub const ROUTE_REGISTER: &'static str = "/register";
pub const ROUTE_LOGIN: &'static str = "/login";
pub const ROUTE_LOGOUT: &'static str = "/logout";

pub const ROUTE_FOOTER: Route = Route::new("/static/footer", "static/footer.html");
pub const ROUTE_NAV: Route = Route::new("/static/footer", "nav/footer.html");
pub const ROUTE_ABOUT: Route = Route::new("/static/about", "static/about.html");
pub const ROUTE_AUTH: Route = Route::new("/static/auth", "static/auth.html");

pub const ROUTE_STATICS: &'static str = "/static/{route}";

pub const STATIC_ROUTES: [Route; 4] = [ROUTE_FOOTER, ROUTE_NAV, ROUTE_ABOUT, ROUTE_AUTH];

fn get_route_context() -> Context {
    let mut routes: HashMap<String, String> = HashMap::new();

    routes.insert("welcome".into(), ROUTE_WELCOME.web_path.into());
    routes.insert("products".into(), ROUTE_PRODUCTS.web_path.into());
    routes.insert("auth".into(), ROUTE_AUTH.web_path.into());
    routes.insert("dashboard".into(), ROUTE_DASHBOARD.web_path.into());
    routes.insert("register".into(), ROUTE_REGISTER.into());
    routes.insert("login".into(), ROUTE_LOGIN.into());
    routes.insert("logout".into(), ROUTE_LOGOUT.into());
    routes.insert("footer".into(), ROUTE_FOOTER.web_path.into());
    routes.insert("nav".into(), ROUTE_NAV.web_path.into());
    routes.insert("about".into(), ROUTE_ABOUT.web_path.into());

    let mut context = Context::new();
    context.insert("routes", &routes);

    context
}
pub static ROUTE_CONTEXT: LazyLock<Context> = LazyLock::new(|| get_route_context());

pub fn render_to_response(render: tera::Result<String>) -> HttpResponse {
    match render {
        Ok(s) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(s),
        Err(err) => {
            eprintln!("error during rendering err:\n{err}");
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(err.to_string())
        }
    }
}

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
