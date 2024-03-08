mod signin;
mod signup;
pub mod guards;

use actix_web::{HttpResponse, web, get};
use serde::Deserialize;

use crate::{
    errors::AppError,
    routes::{signup::registration,
    signin::login
    }, 
};

#[derive(Debug, Deserialize)]
pub struct AuthenticationRequest {
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "password")]
    pub password: String,
}

pub fn config_authentification(cfg: &mut web::ServiceConfig) {
    cfg
        .service(login)
        .service(registration);
}

fn convert<T, E>(res: Result<Result<T, AppError>, E>) -> Result<HttpResponse, AppError> 
where
    T: serde::Serialize,
    E: std::fmt::Debug,
    AppError: From<E>,
{
    res.unwrap().map(|d| HttpResponse::Ok().json(d))
}

#[get("/index")]
pub async fn page() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().body("Hello world!"))
}