use actix_session::Session;
use actix_web::{HttpResponse, Responder, get, post, web::Form};
use serde::{Deserialize, Serialize};

use crate::{
    models::user::User,
    traits::{Renderable, Responseable},
};

#[derive(Serialize, Default)]
pub struct Auth {}
impl Renderable for Auth {
    fn render(&self) -> Result<String, tera::Error> {
        Ok(include_str!("../../html/auth.html").to_string())
    }
}
impl Responseable for Auth {}

#[get("/auth")]
async fn auth_get() -> impl Responder {
    Auth::default().into_response()
}

#[derive(Deserialize)]
struct RegisterForm {
    mail: String,
    phone: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginForm {
    mail: String,
    password: String,
}

#[post("/login")]
pub async fn login_post(form: Form<LoginForm>, session: Session) -> impl Responder {
    let data = form.into_inner();
    let maybe_user = User::verfify_login(&data.mail, &data.password);
    let location = if let Some(id_user) = maybe_user {
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

#[post("/register")]
pub async fn register_post(form: Form<RegisterForm>, session: Session) -> impl Responder {
    eprintln!("Handling registration");
    let data = form.into_inner();
    let maybe_user = User::add_user(&data.mail, &data.phone, &data.password);
    let location = if let Ok(user) = maybe_user {
        println!("adding user worked");
        if let Err(err) = session.insert("id_user", user.id_users) {
            eprintln!("Error during session attribution :{}", err)
        }
        "/"
    } else {
        println!("adding user failed");
        "/auth"
    };
    HttpResponse::SeeOther()
        .append_header(("Location", location))
        .finish()
}
#[get("/logout")]
pub async fn logout_get(session: Session) -> impl Responder {
    if session.get::<i32>("id_user").is_ok() {
        session.remove("id_user");
    }
    HttpResponse::Found()
        .append_header(("Location", "/auth"))
        .finish()
}
