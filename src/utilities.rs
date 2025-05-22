use std::{error::Error, fmt::Display};

use crate::{
    log,
    statics::{APP_STATE, AppState, DB_POOL},
};
use actix_web::HttpResponse;
use diesel::{
    QueryResult,
    r2d2::{ConnectionManager, PooledConnection},
};
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

pub fn handle_optional_query_result<T>(
    query_result: QueryResult<T>,
    on_error_log: impl Display,
) -> DynResult<Option<T>> {
    return match query_result {
        Ok(p) => Ok(Some(p)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(err) => {
            log!("{on_error_log}:{}", err);
            Err(Box::new(err))
        }
    };
}
