use actix_web::web;
use futures::FutureExt;
use actix_web::dev::ServiceRequest;

use crate::{
    errors::AppError, 
    db_connection::PgPool, 
    models::check_token, AuthorizedUser,
};

pub async fn process_token(
    processed_token: String, 
    pool: web::Data<PgPool>, 
    auth: web::Data<AuthorizedUser>
) -> Result<(), AppError> {
    let result = web::block(move|| {
        let conn = &mut pool.get().expect("Ошибка соединения при обработке токена");                
        match check_token(processed_token, conn) {
            Ok(user) => Ok(user.id),
            Err(e) => Err(e),
        }
    })
    .map(|f| f.unwrap())
    .await;

    auth.get_ref().user_id.set(result.expect("Ошибка AuthorizedUser"));

    Ok(())
}

pub fn extract_header_token(request: &ServiceRequest) -> Result<String, AppError> {
    match request.headers().get("authorization") {
        Some(token) => {
            match token.to_str() {
                Ok(processed_token) => Ok(String::from(processed_token)),
                Err(_) => Err(AppError::ErrorProcessingToken),
            }
        },
        None => Err(AppError::NoTokenInHeader)
    }
}

/*
#[post("/login")]
pub async fn login(
    maybe_login_request: Option<web::Json<AuthenticationRequest>>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, AppError> {
    web::block(move|| {
        let conn = &mut pool.get().expect("Ошибка соединения при идентификации");
        match maybe_login_request.unwrap().into_inner().login(conn) {
            Ok(user) => create_token(user, conn),
            Err(e) => Err(e),
        }
    })
    .then(|res| async {convert(res)})
    .await
}
 */
/*
fn extract_header_token(request: &ServiceRequest, conn: &mut PgConnection) -> Result<String, AppError> {
    match request.headers().get("user-token") {
        Some(token) => {
            match token.to_str() {
                Ok(processed_token) => {
                    match check_token(processed_token, conn) {
                        Ok(token) => Ok(token),
                        Err(e) => Err(AppError::from(e)),
                    }
                },
                Err(_) => Err(AppError::ErrorProcessingToken),
            }
        },
        None => Err(AppError::NoTokenInHeader)
    }
}
 */