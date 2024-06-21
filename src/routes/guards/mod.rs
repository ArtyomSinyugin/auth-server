use actix_web::{dev::ServiceRequest, guard::Guard};
use actix_web::web;
use diesel::deserialize::FromSqlRow;
//use diesel::{deserialize::FromSqlRow, sql_types::Integer, expression::AsExpression};

use crate::{
    db_connection::PgPool, AuthorizedUser, errors::AppError, models::check_token,
};

#[derive(PartialEq, Debug, FromSqlRow, Clone, Copy)]
//#[diesel(sql_type = Integer)]
pub enum AccessRights {
    Admin,
    User, 
    Unregistered, 
}

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

pub fn extract_header_token(request: &ServiceRequest) -> Result<String, AppError> {
    match request.headers().get("authorization") {
        Some(token) => match token.to_str() {
            Ok(processed_token) => Ok(String::from(processed_token)),
            Err(_) => Err(AppError::ErrorProcessingToken),
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

    //auth.get_ref()
    //    .user_id
    //    .set(result.expect("Ошибка AuthorizedUser"));
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
