use actix_session::SessionExt;
use actix_web::{
    Error, HttpResponse,
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;
use std::future::{Ready, ready};

use crate::log;
use crate::{
    models::user_model::User,
    routes::{ROUTE_AUTH, ROUTE_WELCOME},
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

        let (is_admin, is_connected) = match User::from_session(&session) {
            Ok(Some(user)) => {
                if user.admin == 0 {
                    log!(
                        "The user: \n{:?} try to access a admin protected route without being an admin req:\n{:?}",
                        &user,
                        &req
                    );
                    (false, true)
                } else {
                    log!("Admin :{:?} access the admin route :{:?}", &user, &req);
                    (true, true)
                }
            }
            Ok(None) => {
                log!(
                    "User tried to access admin protected route without being logged in req:\n{:?}",
                    &req
                );
                (false, false)
            }
            Err(err) => {
                log!(
                    "Error inside the adminMiddleware when requesting the user:\n{:?}",
                    err
                );
                (false, false)
            }
        };

        if !is_admin {
            let (req, _pl) = req.into_parts();

            let location = if is_connected {
                ROUTE_WELCOME.web_path
            } else {
                ROUTE_AUTH.web_path
            };

            let res = HttpResponse::Found()
                .append_header(("Location", location))
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
