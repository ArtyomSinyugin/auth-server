use std::{future::{ready, Ready, Future}, pin::Pin};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

use crate::{routes::guards::{extract_header_token, process_token}, AuthorizedUser, PgPool};

pub(crate) struct Authorization;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ChangeAccessRights<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ChangeAccessRights { service }))
    }
}

pub struct ChangeAccessRights<S> {
    /// The next service to call
    service: S,
}

// This future doesn't have the requirement of being `Send`.
// See: futures_util::future::LocalBoxFuture
type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

// `S`: type of the wrapped service
// `B`: type of the body - try to be generic over the body where possible
impl<S, B> Service<ServiceRequest> for ChangeAccessRights<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    // This service is ready when its next service is ready
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let pool = req.app_data::<actix_web::web::Data<PgPool>>().cloned().unwrap();

        let processed_token = extract_header_token(&req).unwrap();
        let (user_id, username, access_rights) = process_token(processed_token, pool).unwrap();

        let auth = req.app_data::<actix_web::web::Data<AuthorizedUser>>().cloned().unwrap();

        auth.get_ref().user_id.set(user_id);
        let mut _user_name = auth.user_name.lock().unwrap();
        let mut _rights = auth.access_rights.lock().unwrap();
        *_user_name = username;
        *_rights = access_rights;

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}