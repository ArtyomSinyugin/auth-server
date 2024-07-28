use diesel::prelude::*;

use crate::{
    db_ops::{auth::get_user_by_token, PgConnection}, errors::AppError, models::{FetchTimer, NewTask, NewTimer, Task, User}, process_tokens::fetch_token, routes::{OperationsWithJobs, TimerCreateRequest}, schema::{tasks, timers::{self, finished_at}, tokens}
};

pub fn fetch_init_data_for_user(processed_token: String, conn: &mut PgConnection) -> Result<(OperationsWithJobs, Vec<FetchTimer>), AppError> { 
    let token = fetch_token(processed_token.clone(), conn)?;
    let tasks: Vec<Task> = Task::belonging_to(&token).select(Task::as_select()).get_results::<Task>(conn)?;
    let tasks = tasks.iter().map(|task| task.task.clone()).collect::<Vec<String>>();
    let subquery = tokens::table.filter(tokens::token.eq(processed_token)).select(tokens::user_id).into_boxed(); 
    let timers: Vec<FetchTimer> = timers::table.select((timers::task, timers::started_at))
    .filter(timers::user_id.eq_any(subquery).and(finished_at.is_null()))
    .get_results::<FetchTimer>(conn)?;
    Ok(
        (OperationsWithJobs { tasks }, timers)
    )
}

/* pub fn fetch_timers_for_user(processed_token: String, conn: &mut PgConnection) -> Result<Vec<FetchTimer>, AppError> { // необходимо отправлять в том же запросе, что и fetch tasks
    let subquery = tokens::table.filter(tokens::token.eq(processed_token)).select(tokens::user_id).into_boxed(); 
    let timers: Vec<FetchTimer> = timers::table.select((timers::task, timers::started_at))
    .filter(timers::user_id.eq_any(subquery).and(finished_at.is_null()))
    .get_results::<FetchTimer>(conn)?; 
    Ok(timers)
} */

pub fn create_task_for_user_into_db(processed_token: String, task: OperationsWithJobs, conn: &mut PgConnection) -> Result<(), AppError> {  
    let user = get_user_by_token(processed_token, conn)?;
    let task_entry = NewTask {
        task: &task.tasks[0],
        task_group: "DELETE GROUP",  // в будущем группа для заданий будет приходить с клиента
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
        Ok(user) => create_timer_for_task(user, new_timer, conn),
        Err(_) => Err(AppError::OperationCanceled),
    }
}

pub fn create_timer_for_task(user: User, new_timer: TimerCreateRequest, conn: &mut PgConnection) -> Result<(), AppError> {
    let timer_entry = NewTimer {
        user_id: &user.id,
        task: &new_timer.task,
        started_at: &new_timer.started_at,
    };
    match diesel::insert_into(timers::table)
        .values(timer_entry)
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::DatabaseError(e)),
    }
}

pub fn insert_finished_at_to_timer(processed_token: String, timer: TimerCreateRequest, conn: &mut PgConnection) -> Result<(), AppError> {
    let subquery = tokens::table.filter(tokens::token.eq(processed_token)).select(tokens::user_id).into_boxed();                          
    diesel::update(timers::table.filter(timers::started_at.eq(timer.started_at)))
    .filter(timers::user_id.eq_any(subquery))
    .set(timers::finished_at.eq(timer.finished_at))
    .execute(conn)?;
    Ok(())
}

pub fn detele_timer_from_db(processed_token: String, timer_to_delete: TimerCreateRequest, conn: &mut PgConnection) -> Result<(), AppError> {
    let subquery = tokens::table.filter(tokens::token.eq(processed_token)).select(tokens::user_id).into_boxed();                             
    diesel::delete(timers::table.filter(timers::started_at.eq(timer_to_delete.started_at)))
    .filter(timers::user_id.eq_any(subquery))
    .execute(conn)?;
    Ok(())
}

pub fn update_job(processed_token: String, job_to_update: Vec<String>, conn: &mut PgConnection) -> Result<(), AppError> {
    let subquery = tokens::table.filter(tokens::token.eq(processed_token)).select(tokens::user_id).into_boxed();                           
    diesel::update(tasks::table.filter(tasks::task.eq(job_to_update[0].clone()))).filter(tasks::user_id.eq_any(subquery)).set(tasks::task.eq(job_to_update[1].clone())).execute(conn)?;
    // в векторе job_to_update должны быть два названия. Первое - старое название, второе - новое
    Ok(())
}

pub fn delete_task(processed_token: String, task_to_delete: Vec<String>, conn: &mut PgConnection) -> Result<(), AppError> {
    let subquery = tokens::table.filter(tokens::token.eq(processed_token)).select(tokens::user_id).into_boxed();                           
    diesel::delete(tasks::table.filter(tasks::task.eq_any(task_to_delete))).filter(tasks::user_id.eq_any(subquery)).execute(conn)?;
    Ok(())
}