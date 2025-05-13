use actix_session::Session;
use actix_web::{HttpResponse, Responder, get, post, web::Form};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::{
    TERA,
    traits::{Renderable, Responseable},
};

#[derive(Serialize)]
pub struct Auth {}
impl Renderable for Auth {
    fn render(&self) -> Result<String, tera::Error> {
        Ok(include_str!("../../html/auth.html").to_string())
    }
}
impl Responseable for Auth {}

impl Auth {
    pub fn new() -> Self {
        Self {}
    }
}
#[get("/auth")]
async fn auth_get() -> impl Responder {
    Auth::new().into_response()
}
struct LoginForm {}

#[derive(Deserialize)]
struct RegisterForm {
    mail: String,
    phone: String,
    password: String,
}
/*#[post("/login")]
async fn login_post() -> impl Responder {
    todo!()
}

#[post("/register")]
async fn register_post(form: Form<RegisterForm>) -> impl Responder {}*/
