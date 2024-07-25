use diesel::prelude::*;

use crate::{
    db_ops::{auth::get_user_by_token, PgConnection}, errors::AppError, models::{Job, NewTask, Timer, User}, process_tokens::fetch_token, routes::{OperationsWithJobs, TimerCreateRequest}, schema::{jobs, timers, tokens}
};

pub fn create_task_for_user_into_db(processed_token: String, job: OperationsWithJobs, conn: &mut PgConnection) -> Result<(), AppError> {  // здесь даём токен, а не пользователя
    let user = get_user_by_token(processed_token, conn)?;
    let job_entry = NewTask {
        job: &job.tasks[0],
        user_id: &user.id,
    };
    match diesel::insert_into(jobs::table)
        .values(job_entry)
        .execute(conn)
    {
        Ok(_) => {Ok(())},
        Err(e) => {Err(AppError::DatabaseError(e))},
    }
}

pub fn insert_timer_for_user_into_db(processed_token: String, new_timer: TimerCreateRequest, conn: &mut PgConnection) -> Result<(), AppError> {
    match get_user_by_token(processed_token, conn) 
    {
        Ok(user) => create_timer_for_job(user, new_timer, conn),
        Err(_) => Err(AppError::OperationCanceled),
    }
}

pub fn create_timer_for_job(user: User, new_timer: TimerCreateRequest, conn: &mut PgConnection) -> Result<(), AppError> {
    let timer_entry = Timer {
        user_id: &user.id,
        job: &new_timer.job,
        started_at: &new_timer.started_at,
        finished_at: &new_timer.finished_at,
    };
    match diesel::insert_into(timers::table)
        .values(timer_entry)
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::DatabaseError(e)),
    }
}

pub fn fetch_all_tasks_for_user(processed_token: String, conn: &mut PgConnection) -> Result<OperationsWithJobs, AppError> { 
    let token = fetch_token(processed_token, conn)?;
    let vec_jobs: Vec<Job> = Job::belonging_to(&token).select(Job::as_select()).get_results::<Job>(conn)?;
    Ok(
        OperationsWithJobs { tasks: vec_jobs.iter().map(|job| job.job.clone()).collect::<Vec<String>>() }
    )
}

pub fn delete_job(processed_token: String, job_to_delete: Vec<String>, conn: &mut PgConnection) -> Result<(), AppError> {
    let subquery = tokens::table.filter(tokens::token.eq(processed_token)).select(tokens::user_id).into_boxed();                             // задача удалить строку (или строки) с конкретными работами/работой, в которой user_id = user_id токена, в один запрос к БД
    diesel::delete(jobs::table.filter(jobs::job.eq_any(job_to_delete))).filter(jobs::user_id.eq_any(subquery)).execute(conn)?;
    Ok(())
}

pub fn update_job(processed_token: String, job_to_update: Vec<String>, conn: &mut PgConnection) -> Result<(), AppError> {
    let subquery = tokens::table.filter(tokens::token.eq(processed_token)).select(tokens::user_id).into_boxed();                             // задача удалить строку (или строки) с конкретными работами/работой, в которой user_id = user_id токена, в один запрос к БД
    diesel::update(jobs::table.filter(jobs::job.eq(job_to_update[0].clone()))).filter(jobs::user_id.eq_any(subquery)).set(jobs::job.eq(job_to_update[1].clone())).execute(conn)?;
    // в векторе job_to_update должны быть два названия. Первое - старое название, второе - новое
    Ok(())
}