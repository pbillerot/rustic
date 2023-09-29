use std::future::{ready, Ready};

use actix_session::SessionExt;
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    // http,
    Error, HttpResponse, http,
    // HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use crate::router::{get_back, compute_back};

pub struct SilexSession;

impl<S, B> Transform<S, ServiceRequest> for SilexSession
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = SilexSessionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SilexSessionMiddleware { service }))
    }
}
pub struct SilexSessionMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SilexSessionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // https://github.com/TianLangStudio/rust_cms/blob/master/web/src/middleware.rs
        // let path = req.path().to_string();
        let session = &req.get_session();
        log::info!("11 {:?}", session.entries());

        if get_back(&session).is_empty()  { // && path.find("/login").is_none()
            log::warn!("Session expired");
            session.insert("back", "/").unwrap();
            let (request, _pl) = req.into_parts();
            let response = HttpResponse::Found()
                .insert_header((http::header::LOCATION, "/"))
                .finish()
                .map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }
        compute_back(&req.request(), &session);
        log::info!("22 {:?}", session.entries());

        let res = self.service.call(req);
            Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })

    }
}
