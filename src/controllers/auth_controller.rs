use actix_session::Session;
use actix_web::{HttpResponse, Responder, web::Form};
use serde::Deserialize;

use crate::{
    log,
    models::user_model::User,
    routes::{ROUTE_AUTH, ROUTE_CONTEXT, ROUTE_PRODUCTS, ROUTE_VERIFY},
    statics::TERA,
    utilities::{DynResult, render_to_response},
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

pub async fn auth_get(session: Session) -> DynResult<HttpResponse> {
    let maybe_user = User::from_session(&session)?;

    if let Some(user) = maybe_user {
        log!("User {} access auth page", &user);

        let location = if user.verified == 0 {
            ROUTE_VERIFY.web_path
        } else {
            ROUTE_PRODUCTS.web_path
        };

        return Ok(HttpResponse::Found()
            .append_header(("Location", location))
            .finish());
    }

    Ok(render_to_response(
        TERA.render(ROUTE_AUTH.file_path, &ROUTE_CONTEXT),
    ))
}

pub async fn login_post(form: Form<LoginForm>, session: Session) -> DynResult<HttpResponse> {
    let data = form.into_inner();

    let maybe_user = User::verify_login(&data.mail, &data.password)?;
    let Some(user) = maybe_user else {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", ROUTE_AUTH.web_path))
            .finish());
    };
    session.insert("id_user", user.id_user)?;

    let location = if user.verified == 0 {
        ROUTE_VERIFY.web_path
    } else {
        ROUTE_PRODUCTS.web_path
    };

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", location))
        .finish())
}

pub async fn register_post(form: Form<RegisterForm>, session: Session) -> DynResult<HttpResponse> {
    eprintln!("Handling registration");
    let data = form.into_inner();
    let user = User::add_user(&data.mail, &data.phone, &data.password)?;
    session.insert("id_user", user.id_user)?;

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", ROUTE_VERIFY.web_path))
        .finish())
}

pub async fn logout_get(session: Session) -> impl Responder {
    if session.get::<i32>("id_user").is_ok() {
        session.remove("id_user");
    }
    HttpResponse::Found()
        .append_header(("Location", ROUTE_AUTH.web_path))
        .finish()
}
