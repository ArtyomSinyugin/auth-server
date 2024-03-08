use actix_web::{post, web, HttpResponse};
use futures::FutureExt;

use crate::{
    errors::AppError, 
    db_connection::PgPool, 
    models::AuthorizationDatabase,
    routes::{
        AuthenticationRequest,
        convert, 
    }
};

#[post("/login")]
pub async fn login(
    maybe_login_request: Option<web::Json<AuthenticationRequest>>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, AppError> {
    web::block(move|| {
        let conn = &mut pool.get().expect("Ошибка соединения при идентификации");
        match maybe_login_request.unwrap().into_inner().login(conn) {
            Ok(token) => Ok(token),
            Err(e) => Err(e),
        }
    })
    .then(|res| async {convert(res)})
    .await
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