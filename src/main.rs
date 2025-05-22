mod macros;
pub mod routes;
mod schema;
mod statics;
mod utilities;

pub mod middlewares {
    pub mod admin_middleware;
    pub mod auth_middleware;
}

pub mod components {}

pub mod models {
    pub mod category_model;
    pub mod category_product_model;
    pub mod complex_request;
    pub mod order_model;
    pub mod order_product_model;
    pub mod product_model;
    pub mod user_model;
}
pub mod controllers {
    pub mod auth_controller;
    pub mod cart_controller;
    pub mod dashboard_controller;
    pub mod order_controller;
    pub mod products_controller;
    pub mod static_component_controller;
    pub mod welcome_controller;
}

use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::web::{self, delete, get, patch, post, put};
use actix_web::{App, HttpServer, cookie::Key, web::scope};
use controllers::auth_controller::{auth_get, login_post, logout_get, register_post};
use controllers::cart_controller::{self};
use controllers::dashboard_controller::dashboard_get;
use controllers::order_controller;
use controllers::products_controller::{
    product_id_delete, product_id_get, product_id_patch, product_post, product_put_visibility,
    products_get,
};
use controllers::static_component_controller::static_route_get;
use controllers::welcome_controller::welcome_get;
//use middlewares::admin_middleware::AdminMiddleware;
use middlewares::auth_middleware::AuthMiddleware;

use routes::*;
use rustls::ServerConfig;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;
use rustls::pki_types::pem::PemObject;
use statics::APP_STATE;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log!("App start at:{}", utilities::now());
    dotenvy::dotenv().expect("can't load the dotenv");
    let Ok(string_port) = std::env::var("PORT") else {
        panic!("APP_STATE do not existe or is not valide. Valide state are: prod, dev");
    };
    let Ok(port) = string_port.parse() else {
        panic!("Le port n'est pas valide");
    };
    let allow_incoming = match *APP_STATE {
        statics::AppState::Prod => "0.0.0.0",
        statics::AppState::Dev => "127.0.0.1",
    };

    let config = load_rustls_config();

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
            .route(ROUTE_REGISTER, post().to(register_post))
            .route(ROUTE_LOGIN, post().to(login_post))
            .route(ROUTE_STATICS, get().to(static_route_get))
            .route(ROUTE_WELCOME.web_path, get().to(welcome_get))
            .route(ROUTE_PRODUCTS.web_path, get().to(products_get))
            .route(ROUTE_AUTH.web_path, get().to(auth_get))
            .service(
                scope("")
                    .wrap(AuthMiddleware)
                    .route(ROUTE_DASHBOARD.web_path, get().to(dashboard_get))
                    .route(ROUTE_LOGOUT, get().to(logout_get)) //.service(scope("").wrap(AdminMiddleware).route(ROUTE_, route)),
                    .route(ROUTE_EDIT_PRODUCT.web_path, get().to(product_id_get))
                    .route(ROUTE_DELETE_PRODUCT, delete().to(product_id_delete))
                    .route(ROUTE_EDIT_PRODUCT.web_path, patch().to(product_id_patch))
                    .route(ROUTE_PRODUCT_NEW, post().to(product_post))
                    .route(ROUTE_PRODUCT_VISIBILITY, put().to(product_put_visibility))
                    .route(ROUTE_CART.web_path, web::get().to(cart_controller::index))
                    .route(ROUTE_ORDER, put().to(order_controller::update)),
            )
    })
    .bind_rustls_0_23((allow_incoming, port), config)?
    .run()
    .await
}
fn load_rustls_config() -> rustls::ServerConfig {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    // load TLS key/cert files
    let cert_chain = CertificateDer::pem_file_iter("cert.pem")
        .unwrap()
        .flatten()
        .collect();

    let key_der =
        PrivateKeyDer::from_pem_file("key.pem").expect("Could not locate PKCS 8 private keys.");

    ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)
        .unwrap()
}
