use actix_session::Session;
use actix_web::{HttpResponse, Responder, get, post, web::Form};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::{
    TERA,
    traits::{Renderable, Responseable},
};

#[derive(Serialize)]
pub struct Login {}
impl Renderable for Login {
    fn render(&self) -> Result<String, tera::Error> {
        TERA.render("login.html", &Context::new())
    }
}
impl Responseable for Login {}

impl Login {
    pub fn new() -> Self {
        Login {}
    }
}

#[derive(Deserialize)]
pub struct LoginData {
    mail: String,
    password: String,
}
impl LoginData {
    pub fn verify(&self) -> bool {
        (self.mail == "joshuabouchat@gmail.com") && (self.password == "passwd")
    }
}
#[get("/login")]
async fn login_get(session: Session) -> impl Responder {
    if session.get::<i32>("user_id").unwrap().is_some() {
        return HttpResponse::Found()
            .append_header(("Location", "/dashboard"))
            .finish();
    }
    println!("login in");
    Login::new().into_response()
}
#[post("/login")]
async fn login_post(session: Session, form: Form<LoginData>) -> impl Responder {
    let datas = form.into_inner();
    if datas.verify() {
        println!("Logged in");
        let inserting = session.insert("user_id", 1);
        println!("inserting: {:?}", inserting);

        HttpResponse::SeeOther()
            .append_header(("Location", "/dashboard"))
            .finish()
    } else {
        println!("refused");
        HttpResponse::Unauthorized().body("Invalid login")
    }
}
