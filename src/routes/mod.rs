pub mod guards;
mod signin;
mod signup;
pub mod requests;

use actix_web::{web, HttpResponse};
use requests::tasks::{create_task_request, create_task_timer_request, delete_tasks_request, get_all_tasks_for_user};
use serde::{Deserialize, Serialize};

use crate::{
    errors::AppError, models::AccessRights, routes::{signin::login, signup::registration}
};

#[derive(Debug, Deserialize)]
pub struct AuthenticationRequest {
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "password")]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimerCreateRequest {
    #[serde(rename = "job")]
    pub job: String,
    #[serde(rename = "started_at")]
    pub started_at: String,
    #[serde(rename = "finished_at")]
    pub finished_at: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OperationsWithJobs {
    #[serde(rename = "tasks")]
    pub jobs: Vec<String>,
}

pub fn config_authentification(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("auth").guard(AccessRights::guard(AccessRights::Unregistered))
            .service(login)
            .service(registration) 
    );
}

pub fn config_tasks(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("tasks").guard(AccessRights::guard(AccessRights::User))
            .service(get_all_tasks_for_user)
            .service(create_task_timer_request) 
            .service(create_task_request)
            .service(delete_tasks_request)
    );
}

fn convert<T, E>(res: Result<Result<T, AppError>, E>) -> Result<HttpResponse, AppError>
where
    T: serde::Serialize,
    E: std::fmt::Debug,
    AppError: From<E>,
{
    res.unwrap().map(|d| HttpResponse::Ok().json(d))
}