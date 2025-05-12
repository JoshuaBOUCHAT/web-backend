mod traits;
mod components {
    pub mod dashboard;
    pub mod login;
    pub mod welcome;
}
pub mod auth_middleware;

pub use crate::components::welcome::WelcomeBuilder;
use crate::traits::Responseable;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::{
    App, HttpServer, Responder,
    cookie::Key,
    get, post,
    web::{self, scope},
};
use auth_middleware::AuthMiddleware;
use components::{
    dashboard::dashboard_get,
    login::{login_get, login_post},
};
use once_cell::sync::Lazy;
use tera::Tera;
use traits::Renderable;

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
            .service(login_get)
            .service(login_post)
            .service(scope("").wrap(AuthMiddleware).service(dashboard_get))

        //.route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
