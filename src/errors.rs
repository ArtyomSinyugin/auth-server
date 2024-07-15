use actix_web::{error::BlockingError, HttpResponse, ResponseError};
use diesel::result::{
    DatabaseErrorKind::{NotNullViolation, SerializationFailure, UniqueViolation},
    Error::{DatabaseError, NotFound},
};
use serde::Serialize;
use std::fmt;

// создаём собственный список ошибок
#[derive(Debug)]
pub enum AppError {
    UsernameAlreadyInUse,
    SerializationFailure,
    WeakPassword,
    TooLongPassword,
    WrongPassword,
    //    TokenNotFound,
    // ErrorProcessingToken,
    NoTokenInHeader,
    UnauthorizedUser,
    Unreachable,
    NotNullViolation,
    NotFound,
    DatabaseError(diesel::result::Error),
    OperationCanceled,
    HashFailure,
}
// сопоставляем ошибки базы данных с нашим списком
impl From<diesel::result::Error> for AppError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            DatabaseError(UniqueViolation, _) => AppError::UsernameAlreadyInUse,
            DatabaseError(SerializationFailure, _) => AppError::SerializationFailure,
            DatabaseError(NotNullViolation, _) => AppError::NotNullViolation,
            NotFound => AppError::NotFound, // отсюда удалить, обрабатывать из models?
            _ => AppError::DatabaseError(value),
        }
    }
}
// сопоставляем ошибки сервера auth-server с нашим списком
impl From<BlockingError> for AppError {
    fn from(_: BlockingError) -> Self {
        AppError::OperationCanceled
    }
}
// ошибки, которые могут возникнуть при хеширования пароля
impl From<argon2::password_hash::errors::Error> for AppError {
    fn from(_: argon2::password_hash::errors::Error) -> Self {
        AppError::HashFailure
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::UsernameAlreadyInUse => write!(f, "already_registered"),
            AppError::SerializationFailure => write!(f, "read_or_write_db_serialization_error"),
            AppError::WeakPassword => write!(f, "weak_password"),
            AppError::TooLongPassword => write!(f, "too_long_password"),
            AppError::WrongPassword => write!(f, "wrong_password"),
            //            AppError::TokenNotFound => write!(f, "need_to_login"),
            // AppError::ErrorProcessingToken => write!(f, "error_processing_token"),
            AppError::NoTokenInHeader => write!(f, "there_is_no_token_in_the_header"),
            AppError::UnauthorizedUser => write!(f, "authorization_error"),
            AppError::Unreachable => write!(f, "not enough user rights for access"),
            AppError::NotNullViolation => write!(f, "cant_be_null"),
            AppError::NotFound => write!(f, "user_not_found"),
            AppError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AppError::OperationCanceled => write!(f, "database_cancel_operation"),
            AppError::HashFailure => write!(f, "hash_failure"),
        }
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    err: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let err = format!("{}", self);
        let mut builder = match self {
            AppError::UsernameAlreadyInUse => HttpResponse::BadRequest(),
            AppError::WeakPassword => HttpResponse::LengthRequired(),
            AppError::TooLongPassword => HttpResponse::Forbidden(),
            AppError::Unreachable => HttpResponse::Forbidden(),
            AppError::WrongPassword => HttpResponse::BadRequest(),
            AppError::UnauthorizedUser => HttpResponse::Unauthorized(),
            //            AppError::TokenNotFound => HttpResponse::NotFound(),
            AppError::NotNullViolation => HttpResponse::BadRequest(),
            AppError::NotFound => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError(),
        };
        builder.json(ErrorResponse { err })
    }
}
