use actix_web::{dev::ServiceRequest, guard::Guard};
use actix_web::{web, HttpRequest};
use diesel::PgConnection;

use crate::{
    db_ops::PgPool, AuthorizedUser, errors::AppError, models::{AccessRights, User}, auth::get_user_by_token
};

impl AccessRights {
    pub fn guard(access_rights: AccessRights) -> Self {
        access_rights
    }
}

impl Guard for AccessRights {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        let data = &ctx.app_data::<actix_web::web::Data<AuthorizedUser>>().cloned().unwrap();
        let data = data.access_rights.lock().unwrap();
        if *self == *data { true } else { println!("No user rights"); false }
    }
}

pub fn extract_header_token_from_servicerequest(request: &ServiceRequest) -> Result<String, AppError> {
    match request.headers().get("authorization") {
        Some(token) => match token.to_str() {
            Ok(processed_token) => Ok(String::from(processed_token)),
            Err(_) => Err(AppError::Unreachable),
        },
        None => Err(AppError::NoTokenInHeader),
    }
}

pub fn extract_header_token_from_httprequest(request: &HttpRequest) -> Result<String, AppError> {
    match request.headers().get("authorization") {
        Some(token) => match token.to_str() {
            Ok(processed_token) => Ok(String::from(processed_token)),
            Err(_) => Err(AppError::Unreachable),
        },
        None => Err(AppError::NoTokenInHeader),
    }
}

pub fn process_token(
    processed_token: String,
    pool: web::Data<PgPool>,
) -> Result<(uuid::Uuid, String, crate::AccessRights), AppError> {
    let conn = &mut pool.get().expect("Ошибка соединения при обработке токена");
    let result = match check_token(processed_token, conn) {
        Ok(user) => Ok((
            user.id,
            user.username,
            user.access_rights
        )),
        Err(_) => Err(AppError::UnauthorizedUser),
    };

    result
}

pub fn check_token(processed_token: String, conn: &mut PgConnection) -> Result<User, AppError> {
    match get_user_by_token(processed_token, conn)
    {
        Ok(user) => Ok(user),
        Err(_) => Err(AppError::UnauthorizedUser),
    }
}
