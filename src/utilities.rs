use std::{collections::HashMap, error::Error, io::Write};

use crate::{
    log,
    models::user_model::User,
    statics::{APP_STATE, AppState, DB_POOL},
};
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use diesel::r2d2::{ConnectionManager, PooledConnection};
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
