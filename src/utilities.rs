use crate::models::user::User;
use actix_session::Session;
use actix_web::HttpResponse;

pub trait Renderable {
    fn render(&self) -> Result<String, tera::Error>;
}
pub trait Responseable: Renderable {
    fn into_response(&self) -> HttpResponse {
        render_to_response(self.render())
    }
}
pub fn render_to_response(render: tera::Result<String>) -> HttpResponse {
    match render {
        Ok(s) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(s),
        Err(err) => {
            eprintln!("error during rendering err:\n{err}");
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(err.to_string())
        }
    }
}
pub fn is_admin(session: &Session) -> bool {
    if let Ok(maybe_id_user) = session.get("id_user") {
        if let Some(id_user) = maybe_id_user {
            return User::is_user_admin(id_user);
        }
    }
    false
}
pub fn is_connected(session: &Session) -> bool {
    if let Ok(maybe_id_user) = session.get("id_user") {
        if let Some(id_user) = maybe_id_user {
            return User::exist(id_user);
        }
    }
    false
}
pub fn new_internal_error() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}
