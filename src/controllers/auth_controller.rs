use actix_session::Session;
use actix_web::{HttpResponse, Responder, web::Form};
use serde::Deserialize;

use crate::{
    log,
    models::user_model::User,
    routes::{ROUTE_AUTH, ROUTE_CONTEXT, ROUTE_DASHBOARD, ROUTE_PRODUCTS},
    statics::TERA,
    try_or_return,
    utilities::{error_to_http_repsonse, render_to_response},
};

#[derive(Deserialize)]
pub struct RegisterForm {
    mail: String,
    phone: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    mail: String,
    password: String,
}

pub async fn auth_get(session: Session) -> impl Responder {
    let maybe_user = try_or_return!(error_to_http_repsonse(User::from_session(&session)));

    if let Some(user) = maybe_user {
        log!("User {:?} access auth page", &user);
        return HttpResponse::Found()
            .append_header(("Location", ROUTE_PRODUCTS.web_path))
            .finish();
    }

    render_to_response(TERA.render(ROUTE_AUTH.file_path, &ROUTE_CONTEXT))
}

pub async fn login_post(form: Form<LoginForm>, session: Session) -> impl Responder {
    println!("handling login");
    let data = form.into_inner();
    let maybe_user = try_or_return!(
        error_to_http_repsonse(User::verify_login(&data.mail, &data.password)),
        log!("Error during rendering login_post")
    );
    let location = if let Some(user) = maybe_user {
        if let Err(err) = session.insert("id_user", user.id_user) {
            log!(
                "Error during session attribution :{} for user {:?}",
                err,
                &user
            );
        }
        "/"
    } else {
        "/auth"
    };
    HttpResponse::SeeOther()
        .append_header(("Location", location))
        .finish()
}

pub async fn register_post(form: Form<RegisterForm>, session: Session) -> impl Responder {
    eprintln!("Handling registration");
    let data = form.into_inner();
    let maybe_user = User::add_user(&data.mail, &data.phone, &data.password);
    let location = if let Ok(user) = maybe_user {
        println!("adding user worked");
        if let Err(err) = session.insert("id_user", user.id_user) {
            eprintln!("Error during session attribution :{}", err)
        }
        ROUTE_DASHBOARD.web_path
    } else {
        println!("adding user failed");
        ROUTE_AUTH.web_path
    };
    HttpResponse::SeeOther()
        .append_header(("Location", location))
        .finish()
}

pub async fn logout_get(session: Session) -> impl Responder {
    if session.get::<i32>("id_user").is_ok() {
        session.remove("id_user");
    }
    HttpResponse::Found()
        .append_header(("Location", ROUTE_AUTH.web_path))
        .finish()
}
