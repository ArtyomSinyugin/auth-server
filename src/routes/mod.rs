pub mod guards;
mod signin;
mod signup;

use actix_web::{get, web, HttpResponse};
use serde::Deserialize;

use crate::{
    errors::AppError,
    routes::{signin::login, signup::registration},
};

#[derive(Debug, Deserialize)]
pub struct AuthenticationRequest {
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "password")]
    pub password: String,
}

pub fn config_authentification(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(registration);
}

fn convert<T, E>(res: Result<Result<T, AppError>, E>) -> Result<HttpResponse, AppError>
where
    T: serde::Serialize,
    E: std::fmt::Debug,
    AppError: From<E>,
{
    res.unwrap().map(|d| HttpResponse::Ok().json(d))
}

#[get("/main")]
pub async fn page() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().body("Hello main!"))
}

#[get("/characters")]
pub async fn characters() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().body("Hello characters!"))
}

#[get("/for_staff")]
pub async fn for_staff() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().body("Hello staaff!"))
}
