#![feature(unwrap_infallible)]
mod macros;
pub mod routes;
mod schema;
mod statics;
mod utilities;
use actix_web::web::{delete, get, post};
use chrono::{Date, Utc};
use controllers::products_controller::{product_id_delete, product_id_get, products_get};
use controllers::static_component_controller::static_route_get;
use controllers::welcome_controller::welcome_get;
use middlewares::admin_middleware::AdminMiddleware;
use routes::*;
use utilities::now;

pub mod middlewares {
    pub mod admin_middleware;
    pub mod auth_middleware;
}

pub mod components {}

pub mod models {
    pub mod category_model;
    pub mod category_product_model;
    pub mod order_model;
    pub mod order_product_model;
    pub mod product_model;
    pub mod user_model;
}
pub mod controllers {
    pub mod auth_controller;
    pub mod dashboard_controller;
    pub mod products_controller;
    pub mod static_component_controller;
    pub mod welcome_controller;
}

pub use crate::controllers::welcome_controller::WelcomeBuilder;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::{App, HttpServer, cookie::Key, web::scope};
use controllers::auth_controller::{login_post, logout_get, register_post};
use controllers::dashboard_controller::dashboard_get;
use middlewares::auth_middleware::AuthMiddleware;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write, stderr, stdout};
use std::sync::{LazyLock, Mutex};
use tera::Tera;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log!("App start at:{}", utilities::now());
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
            .service(actix_files::Files::new(
                ROUTE_PUBLIC.web_path,
                ROUTE_PUBLIC.file_path,
            ))
            /* .service(actix_files::Files::new(
                ROUTE_IMAGES.web_path,
                ROUTE_IMAGES.file_path,
            ))
            .service(actix_files::Files::new(
                ROUTE_JS.web_path,
                ROUTE_JS.file_path,
            ))*/
            .route(ROUTE_REGISTER, post().to(register_post))
            .route(ROUTE_LOGIN, post().to(login_post))
            .route(ROUTE_STATICS, get().to(static_route_get))
            .route(ROUTE_WELCOME.web_path, get().to(welcome_get))
            .route(ROUTE_PRODUCTS.web_path, get().to(products_get))
            .service(
                scope("")
                    .wrap(AuthMiddleware)
                    .route(ROUTE_DASHBOARD.web_path, get().to(dashboard_get))
                    .route(ROUTE_LOGOUT, get().to(logout_get)) //.service(scope("").wrap(AdminMiddleware).route(ROUTE_, route)),
                    .route(ROUTE_EDIT_PRODUCT.web_path, get().to(product_id_get))
                    .route(ROUTE_DELETE_PRODUCT, delete().to(product_id_delete)),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
