pub mod guards;
mod signin;
mod signup;
pub mod requests;

use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::{
    errors::AppError,
    routes::{signin::login, signup::registration}, AuthorizedUser,
};

#[derive(Debug, Deserialize)]
pub struct AuthenticationRequest {
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "password")]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct TimerCreateRequest {
    #[serde(rename = "job")]
    pub job: String,
    #[serde(rename = "started_at")]
    pub started_at: String,
    #[serde(rename = "finished_at")]
    pub finished_at: String,
}

#[derive(Debug, Deserialize)]
pub struct OperationsWithJobs {
    #[serde(rename = "job")]
    pub jobs: Vec<String>,
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

pub async fn page404() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().body("Page 404! Go home, body"))
}