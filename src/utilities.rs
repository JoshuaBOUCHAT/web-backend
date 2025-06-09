use std::{error::Error, fmt::Display};

use crate::{
    log,
    models::user_model::User,
    statics::{APP_MAIL_BOX, APP_STATE, AppState, DB_POOL, MAILER},
};
use actix_web::HttpResponse;
use chrono::Utc;
use diesel::{
    QueryResult,
    r2d2::{ConnectionManager, PooledConnection},
};
use lettre::{Message, Transport, message::header::ContentType};
pub trait Renderable {
    fn render(&self) -> Result<String, tera::Error>;
}
pub trait Responseable: Renderable {
    fn into_response(self) -> HttpResponse
    where
        Self: std::marker::Sized,
    {
        render_to_response(self.render())
    }
}
pub fn render_to_response(render: tera::Result<String>) -> HttpResponse {
    match render {
        Ok(s) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(s),
        Err(err) => {
            eprintln!("error during rendering err:\n{:?}", err);
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(err.to_string())
        }
    }
}

pub fn get_db() -> Option<PooledConnection<ConnectionManager<diesel::SqliteConnection>>> {
    DB_POOL.get().ok()
}

pub fn now() -> String {
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub trait ExtractHttp<T> {
    fn extract_http(self) -> Result<T, HttpResponse>;
}
impl<T> ExtractHttp<T> for Result<T, Box<dyn Error>> {
    fn extract_http(self) -> Result<T, HttpResponse> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => {
                log!("error:{err}\n");
                let resp = match *APP_STATE {
                    AppState::Dev => HttpResponse::InternalServerError()
                        .body(format!("An error occurred:<br><h2>{}</h2>", err)),
                    AppState::Prod => HttpResponse::NotFound()
                        .body("Nous sommes désolés, une erreur est survenue"),
                };
                Err(resp)
            }
        }
    }
}

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn handle_optional_query_result<T>(
    query_result: QueryResult<T>,
    on_error_log: impl Display,
) -> DynResult<Option<T>> {
    match query_result {
        Ok(p) => Ok(Some(p)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(err) => {
            log!("{on_error_log}:{}", err);
            Err(Box::new(err))
        }
    }
}
pub fn send_mail(destination: &str, subject: &str, body: impl Into<String>) -> DynResult<()> {
    println!("sending mail to {}", destination);
    let msg = Message::builder()
        .from(APP_MAIL_BOX.clone())
        .to(destination.parse().expect("Invalid email address"))
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body.into())?;
    {
        let mailer = MAILER.lock()?;
        mailer.send(&msg)?;
    }

    Ok(())
}
pub fn now_str() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
pub fn add_login_propetry_to_context(
    context: &mut tera::Context,
    session: &actix_session::Session,
) -> DynResult<()> {
    if let Some(user) = User::from_session(session)? {
        context.insert("is_connected", &true);
        context.insert("is_admin", &user.is_admin());
    } else {
        println!("user not found");
        context.insert("is_connected", &false);
        context.insert("is_admin", &false);
    }
    Ok(())
}
