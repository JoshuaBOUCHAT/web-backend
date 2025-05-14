use actix_session::SessionExt;
use actix_web::{
    Error, HttpResponse,
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;
use std::future::{Ready, ready};

use crate::routes::{ROUTE_AUTH, ROUTE_LOGIN, ROUTE_REGISTER};

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
        let maybe_id = session.get::<i32>("id_user").unwrap_or(None);

        let have_id = maybe_id.is_some();
        let path = req.path();
        println!("mid manage req at: {path}");
        let is_auth_page =
            path == ROUTE_AUTH.web_path || path == ROUTE_LOGIN || path == ROUTE_REGISTER;

        if !have_id {
            if is_auth_page {
                println!("authentification");
                let fut = self.service.call(req);
                return Box::pin(async move {
                    let res = fut.await?.map_into_left_body();
                    Ok(res)
                });
            }
            println!("Redirecting to login");

            let (req, _pl) = req.into_parts();
            let res = HttpResponse::Found()
                .append_header(("Location", ROUTE_AUTH.web_path))
                .finish()
                .map_into_right_body();

            let service_response = ServiceResponse::new(req, res);
            return Box::pin(async move { Ok(service_response) });
        }

        println!("path: {path}");
        if path == ROUTE_AUTH.web_path || path == ROUTE_LOGIN || path == ROUTE_REGISTER {
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
