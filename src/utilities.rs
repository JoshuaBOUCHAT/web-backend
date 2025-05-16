use std::{collections::HashMap, io::Write};

use crate::models::user::User;
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{Error, HttpResponse, web};
use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

/*pub async fn from_multipart<'a, T>(mut payload: Multipart) -> T
where
    T: Deserialize<'a>,
{
    let mut map = HashMap::new();
    while let Some(mut field) = payload.try_next().await? {
        let mut map = HashMap::new();


    }
}
async fn save_file_manual(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let Some(content_disposition) = field.content_disposition() else {
            continue;
        };

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        let filepath = format!("./tmp/{filename}");

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::Ok().into())
}*/
