use actix_web::HttpResponse;

use crate::routes::render_to_response;
pub trait Renderable {
    fn render(&self) -> Result<String, tera::Error>;
}
pub trait Responseable: Renderable {
    fn into_response(&self) -> HttpResponse {
        render_to_response(self.render())
    }
}
