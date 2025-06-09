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
    routes::{
        ROUTE_AUTH, ROUTE_CART, ROUTE_DASHBOARD, ROUTE_LOGIN, ROUTE_PRODUCTS, ROUTE_REGISTER,
        ROUTE_VERIFY, ROUTE_WELCOME,
    },
};

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
        let maybe_user = User::from_session(&session).unwrap();

        let path = req.path();
        log!("auth-mid manage req at: {}", path);
        let is_auth_page = path == ROUTE_AUTH.web_path
            || path == ROUTE_LOGIN
            || path == ROUTE_REGISTER
            || path == ROUTE_VERIFY.web_path;

        let is_not_api_route = path == ROUTE_PRODUCTS.web_path
            || path == ROUTE_DASHBOARD.web_path
            || path == ROUTE_CART.web_path
            || path == ROUTE_WELCOME.web_path;

        if maybe_user.is_none() {
            if is_auth_page {
                log!("authentification");
                let fut = self.service.call(req);
                return Box::pin(async move {
                    let res = fut.await?.map_into_left_body();
                    Ok(res)
                });
            }
            let res = if is_not_api_route {
                log!("Redirecting to login");
                HttpResponse::Found()
                    .append_header(("Location", ROUTE_AUTH.web_path))
                    .finish()
            } else {
                HttpResponse::Unauthorized()
                    .body("Vous devez être connecté pour acceder à cette ressource !")
            };
            return response_to_return(res, req);
        };

        // connected user

        // connected user shouldn't access auths page as he is already authenticated
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
