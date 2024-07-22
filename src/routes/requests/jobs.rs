use actix_web::{get, post, web, HttpRequest, HttpResponse};
use futures::FutureExt;
use crate::{
    db_ops::{process_jobs::{delete_job, insert_job_for_user_into_db, insert_timer_for_user_into_db}, PgPool}, errors::AppError, process_jobs::{fetch_all_jobs_for_user, update_job}, routes::{convert, guards::extract_header_token_from_httprequest, OperationsWithJobs}
};

use super::TimerCreateRequest;

#[get("/get_all_jobs_for_user")]
pub async fn get_all_jobs_for_user(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let token = match extract_header_token_from_httprequest(&req) {
        Ok(value) => value.to_string(),
        Err(e) => return Err(e)
    };
    web::block(move || {
        let conn = &mut pool.get().expect("Ошибка соединения при создании таймера");
        match fetch_all_jobs_for_user(token, conn) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    })
    .then(|res| async { convert(res) })
    .await
}

#[post("/create_job_timer")]
pub async fn create_job_timer(
    req: HttpRequest,
    new_timer_from_client: Option<web::Json<TimerCreateRequest>>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let token = match extract_header_token_from_httprequest(&req) {
        Ok(value) => value.to_string(),
        Err(e) => return Err(e)
    };
    web::block(move || {
        let conn = &mut pool.get().expect("Ошибка соединения при создании таймера");
        let data = new_timer_from_client.unwrap().into_inner();
        match insert_timer_for_user_into_db(token, data, conn) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    })
    .then(|res| async { convert(res) })
    .await
}

#[post("/create_job")]
pub async fn create_job(
    req: HttpRequest,
    new_job_from_client: Option<web::Json<OperationsWithJobs>>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let token = match extract_header_token_from_httprequest(&req) {
            Ok(value) => value.to_string(),
            Err(e) => return Err(e)
        };
    web::block(move || {
        let conn = &mut pool.get().expect("Ошибка соединения при создании работы");
        let data: OperationsWithJobs = new_job_from_client.unwrap().into_inner();
        match insert_job_for_user_into_db(token, data, conn)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    })
    .then(|res| async { convert(res) })
    .await
}

#[post("/delete_job_request")]
pub async fn delete_job_request(
    req: HttpRequest,
    new_job_from_client: Option<web::Json<OperationsWithJobs>>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let token = match extract_header_token_from_httprequest(&req) {
            Ok(value) => value.to_string(),
            Err(e) => return Err(e)
        };
    web::block(move || {
        let conn = &mut pool.get().expect("Ошибка соединения при создании работы");
        let data = new_job_from_client.unwrap().into_inner().jobs;
        match delete_job(token, data, conn)
        {
            Ok(string) => Ok(string),
            Err(e) => Err(e),
        }
    })
    .then(|res| async { convert(res) })
    .await
}

#[post("/update_job_request")]
pub async fn update_job_request(
    req: HttpRequest,
    new_job_from_client: Option<web::Json<OperationsWithJobs>>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let token = match extract_header_token_from_httprequest(&req) {
            Ok(value) => value.to_string(),
            Err(e) => return Err(e)
        };
    web::block(move || {
        let conn = &mut pool.get().expect("Ошибка соединения при создании работы");
        let data = new_job_from_client.unwrap().into_inner().jobs;
        match update_job(token, data, conn)
        {
            Ok(string) => Ok(string),
            Err(e) => Err(e),
        }
    })
    .then(|res| async { convert(res) })
    .await
}