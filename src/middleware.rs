use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use uuid::Uuid;

use crate::{
    models::AccessRights, 
    routes::guards::{extract_header_token_from_servicerequest, process_token}, 
    AuthorizedUser, 
    PgPool
};

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

impl<S, B> Service<ServiceRequest> for ChangeAccessRights<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    // This service is ready when its next service is ready
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let processed_token = match extract_header_token_from_servicerequest(&req) {
            Ok(string) => string,
            Err(_) => { dbg!("No token in the header"); "".to_string() },
        };
        
        let pool: actix_web::web::Data<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>>> = req.app_data::<actix_web::web::Data<PgPool>>().cloned().unwrap();
 
        let auth = req.app_data::<actix_web::web::Data<AuthorizedUser>>().cloned().unwrap();

        {
            let mut user_id_state = auth.user_id.lock().unwrap();
            let mut user_name_state = auth.user_name.lock().unwrap();
            let mut access_rights_state = auth.access_rights.lock().unwrap();
     
            match process_token(processed_token, pool){
                Ok(result) => (*user_id_state, *user_name_state, *access_rights_state) = result,
                Err(_) => (*user_id_state, *user_name_state, *access_rights_state) = (Uuid::nil(), "".to_string(), AccessRights::Unregistered),
            };
        }  // here we do mutex unlock for AuthorizedUser state to use it in guards

        self.service.call(req)
    }
}
