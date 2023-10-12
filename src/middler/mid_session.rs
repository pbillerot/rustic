use std::future::{ready, Ready};

use actix_session::SessionExt;
use actix_web::{
    body::EitherBody,
    dev::{self, Path, ResourceDef, Service, ServiceRequest, ServiceResponse, Transform},
    http,
    // HttpResponse,
    // http,
    Error,
    HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use crate::{
    middler::{flash::FlashMessage, set_flash},
    router::{compute_back, get_back},
};

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

        let session = &req.get_session();
        let response_error = HttpResponse::Found()
            .insert_header((http::header::LOCATION, "/"))
            .finish()
            .map_into_right_body();

        let path = req.path().to_string();
        log::info!("path {:?}", &path);

        // CTRL ACCES APP TABLE VIEW FORM
        if path != "/" && path.find("/static/").is_none() && path.find("/lexic/").is_none() {
            match &req.request().match_pattern() {
                Some(s) => {
                    log::info!("pattern {:?}", s);
                    let resource = ResourceDef::prefix(s);
                    let mut path_resoource = Path::new(req.path());
                    if resource.capture_match_info(&mut path_resoource) {
                        // log::info!("resource {:?}", &path);
                        match path_resoource.get("appid") {
                            Some(appid) => {
                                log::info!("appid {}", appid);
                            }
                            None => {
                                set_flash(
                                    &session,
                                    FlashMessage::error(
                                        format!("app not found on {:?}", &path_resoource).as_str(),
                                    ),
                                )
                                .unwrap();
                                return Box::pin(async move {
                                    Ok(ServiceResponse::new(req.request().clone(), response_error))
                                });
                            }
                        };
                        match path_resoource.get("tableid") {
                            Some(s) => {
                                log::info!("tableid {}", s);
                            }
                            None => {}
                        };
                        match path_resoource.get("viewid") {
                            Some(s) => {
                                log::info!("viewid {}", s);
                            }
                            None => {}
                        };
                        match path_resoource.get("formid") {
                            Some(s) => {
                                log::info!("formid {}", s);
                            }
                            None => {}
                        };
                    }
                }
                None => {}
            };
        };

        if path.find("/static/").is_none() && path.find("/lexic/").is_none() {
            // log::info!("11 {:?}", session.entries());

            if get_back(&session).is_empty() {
                // && path.find("/login").is_none()
                log::warn!("Session expired {}", path);
                session.insert("back", "/").unwrap();

                set_flash(
                    &session,
                    FlashMessage::error(format!("Session expired").as_str()),
                )
                .unwrap();

                // return Box::pin(async { Ok(ServiceResponse::new(*req.request(), response_error)) });
                return Box::pin(async move {
                    Ok(ServiceResponse::new(req.request().clone(), response_error))
                });
            }
            compute_back(&req.request(), &session);
            // log::info!("22 {:?}", session.entries());
        }
        let res = self.service.call(req);
        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}
