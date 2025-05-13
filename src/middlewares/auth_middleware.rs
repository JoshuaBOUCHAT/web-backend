use actix_session::SessionExt;
use actix_web::{
    Error, HttpResponse,
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;
use std::future::{Ready, ready};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareImpl<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareImpl { service }))
    }
}

pub struct AuthMiddlewareImpl<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareImpl<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        let is_logged_in = session.get::<i32>("id_user").unwrap_or(None).is_some();

        if !is_logged_in {
            println!("🔒 Redirecting to login");

            let (req, _pl) = req.into_parts();
            let res = HttpResponse::Found()
                .append_header(("Location", "/auth"))
                .finish()
                .map_into_right_body(); // 👈 wrap as EitherBody::Right

            let service_response = ServiceResponse::new(req, res);
            return Box::pin(async move { Ok(service_response) });
        }
        let path = req.path();
        if path == "/auth" || path == "/login" || path == "/register" {
            let (req, _pl) = req.into_parts();
            let res = HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish()
                .map_into_right_body();

            let service_response = ServiceResponse::new(req, res);
            return Box::pin(async move { Ok(service_response) });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?.map_into_left_body();
            Ok(res)
        })
    }
}
