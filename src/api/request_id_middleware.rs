use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use futures::future::{ok, Ready};
use futures::Future;
use mqtt2influx_core::utils::generate_random_token;
use std::pin::Pin;
use tracing::info_span;
use tracing_futures::Instrument;

pub static REQUEST_ID_LENGTH: usize = 10usize;

#[derive(Clone, Debug)]
pub struct RequestIdType(pub String);

#[derive(Default)]
pub struct RequestId;

impl<S, B> Transform<S, ServiceRequest> for RequestId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestIdMiddleware { service })
    }
}

pub struct RequestIdMiddleware<S> {
    service: S,
}

#[allow(clippy::type_complexity)]
impl<S, B> Service<ServiceRequest> for RequestIdMiddleware<S>
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
        let id = generate_random_token(REQUEST_ID_LENGTH);
        let req_id = RequestIdType(id.clone());
        req.extensions_mut().insert(req_id);

        let fut = self.service.call(req);
        Box::pin(
            async move {
                let res = fut.await?;
                Ok(res)
            }
            .instrument(info_span!("request", request_id = %id)),
        )
    }
}
