use actix_web::HttpResponse;
pub trait Renderable {
    fn render(&self) -> Result<String, tera::Error>;
}
pub trait Responseable: Renderable {
    fn into_response(&self) -> HttpResponse {
        match self.render() {
            Ok(s) => HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(s),
            Err(err) => {
                eprintln!("error during rendering");
                HttpResponse::InternalServerError()
                    .content_type("text/html; charset=utf-8")
                    .body(err.to_string())
            }
        }
    }
}
