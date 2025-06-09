use actix_session::SessionExt;
use actix_web::{
    Error, HttpResponse,
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;
use std::future::{Ready, ready};

use crate::{
    log,
    models::user_model::User,
    routes::{ROUTE_AUTH, ROUTE_LOGIN, ROUTE_REGISTER, ROUTE_VERIFY},
};

pub struct AdminMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AdminMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AdminMiddlewareImpl<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminMiddlewareImpl { service }))
    }
}

pub struct AdminMiddlewareImpl<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AdminMiddlewareImpl<S>
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
        let maybe_user = User::from_session(&session).unwrap();

        let path = req.path();
        println!("mid manage req at: {path}");
        let is_auth_page = path == ROUTE_AUTH.web_path
            || path == ROUTE_LOGIN
            || path == ROUTE_REGISTER
            || path == ROUTE_VERIFY.web_path;

        let Some(user) = maybe_user else {
            log!(
                "La route admin: {} a pu être acceder sans être authentifier",
                path
            );
            let res = HttpResponse::InternalServerError().body("une erreur est survenue !");
            return response_to_return(res, req);
        };
        if !user.is_admin() {
            let res = HttpResponse::Unauthorized()
                .body("Vous n'etes pas autorisée à consulter cette ressource veillez vous conecter en tant qu'administrateur !");

            return response_to_return(res, req);
        }

        println!("path: {path}");
        if is_auth_page {
            let res = HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish();

            return response_to_return(res, req);
        }
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?.map_into_left_body();
            Ok(res)
        })
    }
}
fn response_to_return<B>(
    response: HttpResponse,
    req: ServiceRequest,
) -> LocalBoxFuture<'static, Result<ServiceResponse<EitherBody<B>>, Error>>
where
    B: MessageBody + 'static,
{
    let (req, _pl) = req.into_parts();
    let service_response = ServiceResponse::new(req, response.map_into_right_body());
    Box::pin(async move { Ok(service_response) })
}
