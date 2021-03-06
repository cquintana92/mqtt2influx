use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::time::Instant;
use tracing::{event, Level};

#[derive(Default)]
pub struct RequestLogger {
    ignored_paths: Vec<String>,
}

impl RequestLogger {
    pub fn new_with_ignored_paths(paths: Vec<String>) -> Self {
        Self { ignored_paths: paths }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestLoggerMiddleware {
            service,
            ignored_paths: self.ignored_paths.clone(),
        })
    }
}

pub struct RequestLoggerMiddleware<S> {
    service: S,
    ignored_paths: Vec<String>,
}

impl<S> RequestLoggerMiddleware<S> {
    fn should_log(&self, path: &str) -> bool {
        !self.ignored_paths.iter().any(|p| p == path)
    }
}

#[allow(clippy::type_complexity)]
impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();
        let should_log = self.should_log(&path);
        let method = format!("{}", req.method());

        let fut = self.service.call(req);
        Box::pin(async move {
            if should_log {
                event!(Level::DEBUG, %method, %path);
            }
            let start = Instant::now();
            let res = fut.await?;
            let elapsed = start.elapsed();
            let status = res.status();
            if should_log {
                event!(Level::INFO, duration = ?elapsed, %method, %path, status = ?status);
            }
            Ok(res)
        })
    }
}
