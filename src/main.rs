extern crate public;

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
    pub mod verify_model;
}
pub mod controllers {
    pub mod auth_controller;
    pub mod cart_controller;
    pub mod category_controller;
    pub mod dashboard_controller;
    pub mod order_controller;
    pub mod products_controller;
    pub mod static_component_controller;
    pub mod verify_controller;
    pub mod welcome_controller;
}

use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};

use actix_web::{App, HttpServer, cookie::Key, web::scope};

use routes::*;
use rustls::ServerConfig;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;
use rustls::pki_types::pem::PemObject;
use statics::APP_STATE;

use crate::middlewares::admin_middleware;
use crate::middlewares::auth_middleware;

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
    println!("Opening the application at {}:{}", allow_incoming, port);

    HttpServer::new(move || {
        let sessionmiddleware =
            SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(actix_web::cookie::time::Duration::weeks(2)),
                )
                .build();
        log!("sessionmidlleware built");
        App::new()
            .wrap(sessionmiddleware)
            .service(actix_files::Files::new(
                ROUTE_PUBLIC.web_path,
                ROUTE_PUBLIC.file_path,
            ))
            .configure(configure_guess_routes)
            .service(
                scope("")
                    .wrap(auth_middleware::AuthMiddleware)
                    .configure(configure_auth_routes)
                    .service(
                        scope("")
                            .wrap(admin_middleware::AdminMiddleware)
                            .configure(configure_admin_routes),
                    ),
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
        .expect("Impossible de localiser le fichier cert.pem")
        .flatten()
        .collect();

    let key_der =
        PrivateKeyDer::from_pem_file("key.pem").expect("Could not locate PKCS 8 private keys.");

    ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)
        .unwrap()
}
