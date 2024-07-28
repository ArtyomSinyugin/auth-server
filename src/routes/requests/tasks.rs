use actix_web::{get, post, web, HttpRequest, HttpResponse};
use futures::FutureExt;
use crate::{
    broadcast::{Broadcaster, ClientEvents}, db_ops::{process_tasks::{create_task_for_user_into_db, delete_task, insert_timer_for_user_into_db}, PgPool}, errors::AppError, process_tasks::{detele_timer_from_db, fetch_init_data_for_user, insert_finished_at_to_timer, update_job}, routes::{convert, guards::extract_header_token_from_httprequest, OperationsWithJobs}
};

use super::TimerCreateRequest;

#[get("/get_init_data_for_user")]
pub async fn get_init_data_for_user(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    //fetch_timers_for_user
    let token = match extract_header_token_from_httprequest(&req) {
        Ok(value) => value.to_string(),
        Err(e) => return Err(e)
    };
    web::block(move || {
        let conn = &mut pool.get().expect("Ошибка соединения при создании таймера");
        match fetch_init_data_for_user(token, conn) {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    })
    .then(|res| async { convert(res) })
    .await
}

#[post("/create_task_request")]
pub async fn create_task_request(
    req: HttpRequest,
    create_task: Option<web::Json<OperationsWithJobs>>,
    broadcaster: web::Data<Broadcaster>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let token = match extract_header_token_from_httprequest(&req) {
            Ok(value) => value.to_string(),
            Err(e) => return Err(e)
        };
    let create_task = create_task.unwrap();
    let task_to_db = create_task.clone();
    web::block(move || {
        let conn = &mut pool.get().expect("Ошибка соединения при создании работы");
        match create_task_for_user_into_db(token, task_to_db, conn)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    })
    .then(|res| async { 
        if let Ok(_) = res.as_ref().unwrap() {
            let msg = create_task.into_inner();
            let msg = msg.tasks[0].as_str();
            broadcaster.broadcast(msg, ClientEvents::CreateTask).await;
        }
        convert(res)
    })
    .await
}

#[post("/create_task_timer_request")]
pub async fn create_task_timer_request(
    req: HttpRequest,
    new_timer_from_client: Option<web::Json<TimerCreateRequest>>,
    broadcaster: web::Data<Broadcaster>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let token = match extract_header_token_from_httprequest(&req) {
        Ok(value) => value.to_string(),
        Err(e) => return Err(e)
    };
    let new_timer_from_client = new_timer_from_client.unwrap();
    let new_timer_from_client_to_db = new_timer_from_client.clone();
    web::block(move || {
        let conn = &mut pool.get().expect("Ошибка соединения при создании таймера");
        let data = new_timer_from_client_to_db;
        match insert_timer_for_user_into_db(token, data, conn) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    })
    .then(|res| async { 
        if let Ok(_) = res.as_ref().unwrap() {
            if new_timer_from_client.task.as_str() == "new_day_timer!^%$(^J:" {
                let msg = new_timer_from_client.started_at.to_string();
                broadcaster.broadcast(&msg, ClientEvents::StartDayTime).await;
            } else {
                let msg = format!("{},{}", new_timer_from_client.started_at, new_timer_from_client.task);
                broadcaster.broadcast(&msg, ClientEvents::StartTask).await;
            }
        }
        convert(res)
     })
    .await
}

#[post("/insert_finished_at_to_timer")]
pub async fn insert_finished_at_to_timer_request(
    req: HttpRequest,
    new_timer_from_client: Option<web::Json<TimerCreateRequest>>,
    broadcaster: web::Data<Broadcaster>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let token = match extract_header_token_from_httprequest(&req) {
        Ok(value) => value.to_string(),
        Err(e) => return Err(e)
    };
    let new_timer_from_client = new_timer_from_client.unwrap();
    let new_timer_from_client_to_db = new_timer_from_client.clone();
    web::block(move || {
        let conn = &mut pool.get().expect("Ошибка соединения при создании таймера");
        let data = new_timer_from_client_to_db;
        match insert_finished_at_to_timer(token, data, conn) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    })
    .then(|res| async { 
        if let Ok(_) = res.as_ref().unwrap() {
            if new_timer_from_client.task.as_str() == "new_day_timer!^%$(^J:" {
                let msg = "0".to_string();
                broadcaster.broadcast(&msg, ClientEvents::StopDayTime).await;
            } else {
                let msg = new_timer_from_client.task.to_string();
                broadcaster.broadcast(&msg, ClientEvents::StopTask).await;
            }
        }
        convert(res)
     })
    .await
}

#[post("/delete_task_timer_request")]  // то же, что и insert_finished_at_to_timer_request
pub async fn delete_task_timer_request(
    req: HttpRequest,
    delete_timer: Option<web::Json<TimerCreateRequest>>,
    broadcaster: web::Data<Broadcaster>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let token = match extract_header_token_from_httprequest(&req) {
        Ok(value) => value.to_string(),
        Err(e) => return Err(e)
    };
    let delete_timer = delete_timer.unwrap();
    let delete_timer_from_db = delete_timer.clone();
    web::block(move || {
        let conn = &mut pool.get().expect("Ошибка соединения при создании таймера");
        let data = delete_timer_from_db;
        match detele_timer_from_db(token, data, conn) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    })
    .then(|res| async { 
        if let Ok(_) = res.as_ref().unwrap() {
            if delete_timer.task.as_str() == "new_day_timer!^%$(^J:" {
                let msg = "0".to_string();
                broadcaster.broadcast(&msg, ClientEvents::StopDayTime).await;
            } else {
                let msg = delete_timer.task.to_string();
                broadcaster.broadcast(&msg, ClientEvents::StopTask).await;
            }
        }
        convert(res)
     })
    .await
}

#[post("/update_task_request")]
pub async fn update_task_request(
    req: HttpRequest,
    update_task_name: Option<web::Json<OperationsWithJobs>>,
    broadcaster: web::Data<Broadcaster>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let token = match extract_header_token_from_httprequest(&req) {
            Ok(value) => value.to_string(),
            Err(e) => return Err(e)
        };
    let update_task_name = update_task_name.unwrap().tasks.clone();
    let update_task_name_in_db = update_task_name.clone();
    web::block(move || {
        let conn = &mut pool.get().expect("Ошибка соединения при создании работы");
        match update_job(token, update_task_name_in_db, conn)
        {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    })
    .then(|res| async { 
        if let Ok(_) = res.as_ref().unwrap() {
            let msg = format!("{},{}", update_task_name[0], update_task_name[1]);
            broadcaster.broadcast(&msg, ClientEvents::UpdateTask).await;
        }
        convert(res) 
    })
    .await
}

#[post("/delete_tasks_request")]
pub async fn delete_tasks_request(
    req: HttpRequest,
    delete_tasks_request: Option<web::Json<OperationsWithJobs>>,
    broadcaster: web::Data<Broadcaster>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let token = match extract_header_token_from_httprequest(&req) {
            Ok(value) => value.to_string(),
            Err(e) => return Err(e)
        };
    let delete_tasks_request = delete_tasks_request.unwrap().tasks.clone();
    let delete_task_to_db = delete_tasks_request.clone();
    web::block(move || {
        let conn = &mut pool.get().expect("Ошибка соединения при создании работы");
        match delete_task(token, delete_task_to_db, conn)
        {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    })
    .then(|res| async { 
        if let Ok(_) = res.as_ref().unwrap() {
            let mut msg = String::new();
            for task in delete_tasks_request {
                let prepare_msg = format!("{},", task);
                msg.push_str(&prepare_msg)
            }
            broadcaster.broadcast(&msg, ClientEvents::DeleteTasks).await;
        }
        convert(res) 
     })
    .await
}