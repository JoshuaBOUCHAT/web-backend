mod schema;
mod traits;
mod components {
    pub mod dashboard;
    pub mod welcome;
}
pub mod middlewares {
    pub mod auth_middleware;
}

pub mod models {
    pub mod category;
    pub mod category_product;
    pub mod order;
    pub mod order_product;
    pub mod product;
    pub mod user;
}
pub mod controller {
    pub mod auth;
}

pub use crate::components::welcome::WelcomeBuilder;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::{App, HttpServer, cookie::Key, web::scope};
use components::dashboard::dashboard_get;
use controller::auth::{auth_get, login_post, register_post};
use middlewares::auth_middleware::AuthMiddleware;
use once_cell::sync::Lazy;
use tera::Tera;

static TERA: Lazy<Tera> = Lazy::new(|| {
    let tera = Tera::new("html/*.html").expect("Failed to load templates");
    println!(
        "Loaded templates: {:?}",
        tera.get_template_names().collect::<Vec<_>>()
    );
    tera
});
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

fn establish_connection() -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new("db.sqlite");
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

static DB_POOL: Lazy<DbPool> = Lazy::new(|| establish_connection());

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let sessionmiddleware =
            SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                .cookie_secure(false)
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(actix_web::cookie::time::Duration::weeks(2)),
                )
                .build();
        App::new()
            .wrap(sessionmiddleware)
            .service(actix_files::Files::new("/css", "public/css"))
            .service(actix_files::Files::new("/img", "public/images"))
            .service(actix_files::Files::new("/js", "public/js"))
            .service(register_post)
            .service(login_post)
            .service(auth_get)
            .service(scope("").wrap(AuthMiddleware).service(dashboard_get))

        //.route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
