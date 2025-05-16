use actix_session::Session;
use actix_web::{HttpResponse, Responder, web::Form};
use serde::Deserialize;
use tera::Tera;

use crate::{
    TERA,
    models::user_model::User,
    routes::{ROUTE_AUTH, ROUTE_CONTEXT, ROUTE_DASHBOARD, ROUTE_PRODUCTS},
    utilities::{is_connected, render_to_response},
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
    if is_connected(&session) {
        return HttpResponse::Found()
            .append_header(("Location", ROUTE_PRODUCTS.web_path))
            .finish();
    }

    render_to_response(TERA.render(ROUTE_AUTH.file_path, &ROUTE_CONTEXT))
}

pub async fn login_post(form: Form<LoginForm>, session: Session) -> impl Responder {
    println!("handling login");
    let data = form.into_inner();
    let maybe_user = User::verfify_login(&data.mail, &data.password);
    let location = if let Some(id_user) = maybe_user {
        println!("There is a user in the session");
        if let Err(err) = session.insert("id_user", id_user) {
            eprintln!("Error during session attribution :{}", err)
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
