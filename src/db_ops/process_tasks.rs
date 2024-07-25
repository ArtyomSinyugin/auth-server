use diesel::prelude::*;

use crate::{
    db_ops::{auth::get_user_by_token, PgConnection}, errors::AppError, models::{Task, NewTask, Timer, User}, process_tokens::fetch_token, routes::{OperationsWithJobs, TimerCreateRequest}, schema::{tasks, timers, tokens}
};

pub fn create_task_for_user_into_db(processed_token: String, task: OperationsWithJobs, conn: &mut PgConnection) -> Result<(), AppError> {  // здесь даём токен, а не пользователя
    let user = get_user_by_token(processed_token, conn)?;
    let task_entry = NewTask {
        task: &task.tasks[0],
        user_id: &user.id,
    };
    match diesel::insert_into(tasks::table)
        .values(task_entry)
        .execute(conn)
    {
        Ok(_) => {Ok(())},
        Err(e) => {Err(AppError::DatabaseError(e))},
    }
}

pub fn insert_timer_for_user_into_db(processed_token: String, new_timer: TimerCreateRequest, conn: &mut PgConnection) -> Result<(), AppError> {
    match get_user_by_token(processed_token, conn) 
    {
        Ok(user) => {
            if new_timer.finished_at.is_empty() {
                create_timer_for_task(user, new_timer, conn)
            } else {
                intest_finished_at_to_timer(user, new_timer, conn)
            }

        },
        Err(_) => Err(AppError::OperationCanceled),
    }
}

pub fn intest_finished_at_to_timer(user: User, new_timer: TimerCreateRequest, conn: &mut PgConnection) -> Result<(), AppError> {
    match diesel::update(timers::table).filter(timers::user_id.eq(user.id).and(timers::started_at.eq(new_timer.started_at)))
    .set(timers::finished_at.eq(new_timer.finished_at))
    .execute(conn)
{
    Ok(_) => Ok(()),
    Err(e) => Err(AppError::DatabaseError(e)),
}
}

pub fn create_timer_for_task(user: User, new_timer: TimerCreateRequest, conn: &mut PgConnection) -> Result<(), AppError> {
    let timer_entry = Timer {
        user_id: &user.id,
        task: &new_timer.task,
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
    let vec_tasks: Vec<Task> = Task::belonging_to(&token).select(Task::as_select()).get_results::<Task>(conn)?;
    Ok(
        OperationsWithJobs { tasks: vec_tasks.iter().map(|task| task.task.clone()).collect::<Vec<String>>() }
    )
}

pub fn delete_task(processed_token: String, task_to_delete: Vec<String>, conn: &mut PgConnection) -> Result<(), AppError> {
    let subquery = tokens::table.filter(tokens::token.eq(processed_token)).select(tokens::user_id).into_boxed();                             // задача удалить строку (или строки) с конкретными работами/работой, в которой user_id = user_id токена, в один запрос к БД
    diesel::delete(tasks::table.filter(tasks::task.eq_any(task_to_delete))).filter(tasks::user_id.eq_any(subquery)).execute(conn)?;
    Ok(())
}

pub fn update_job(processed_token: String, job_to_update: Vec<String>, conn: &mut PgConnection) -> Result<(), AppError> {
    let subquery = tokens::table.filter(tokens::token.eq(processed_token)).select(tokens::user_id).into_boxed();                             // задача удалить строку (или строки) с конкретными работами/работой, в которой user_id = user_id токена, в один запрос к БД
    diesel::update(tasks::table.filter(tasks::task.eq(job_to_update[0].clone()))).filter(tasks::user_id.eq_any(subquery)).set(tasks::task.eq(job_to_update[1].clone())).execute(conn)?;
    // в векторе job_to_update должны быть два названия. Первое - старое название, второе - новое
    Ok(())
}