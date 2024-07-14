use actix_web::{post, web, HttpResponse};
use futures::FutureExt;

use crate::{
    db_connection::PgPool,
    errors::AppError,
    models::AuthorizationDatabase,
    routes::{convert, AuthenticationRequest},
};

#[post("/registration")]
pub async fn registration(
    maybe_registration_request: Option<web::Json<AuthenticationRequest>>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let x = 5;
    dbg!(x);
    web::block(move || {
        let conn = &mut pool.get().expect("Ошибка соединения при регистрации");
        match maybe_registration_request
            .unwrap()
            .into_inner()
            .registration(conn)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    })
    .then(|res| async { convert(res) })
    .await
}
