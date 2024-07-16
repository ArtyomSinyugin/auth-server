use diesel::prelude::*;

use crate::{
    db_ops::{auth::get_user_by_token, PgConnection}, errors::AppError, models::{NewJob, Timer, User}, routes::{InsertJob, TimerCreateRequest}, schema::{jobs, timers}
};

pub fn insert_job_for_user_into_db(processed_token: String, job: InsertJob, conn: &mut PgConnection) -> Result<(), AppError> {  // здесь даём токен, а не пользователя
    let user = get_user_by_token(processed_token, conn)?;
    let job_entry = NewJob {
        job: &job.job,
        user_id: &user.id,
    };
    match diesel::insert_into(jobs::table)
        .values(job_entry)
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::from(e)),
    }
}

pub fn insert_timer_for_user_into_db(processed_token: String, new_timer: TimerCreateRequest, conn: &mut PgConnection) -> Result<(), AppError> {
    match get_user_by_token(processed_token, conn) 
    {
        Ok(user) => create_timer_for_job(user, new_timer, conn),
        Err(e) => Err(AppError::from(e)),
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
        Err(e) => Err(AppError::from(e)),
    }
}

pub fn fetch_all_jobs_for_user(processed_token: String, conn: &mut PgConnection) -> Result<Vec<String>, AppError> {
    match get_user_by_token(processed_token, conn) 
    {
        Ok(user) => {
            match jobs::table
            .select(jobs::job)
            .filter(jobs::user_id.eq(user.id))
            .get_results(conn) {
                    Ok(jobs) => Ok(jobs),
                    Err(e) => Err(AppError::from(e)),
                }

                
        },
        Err(e) => Err(AppError::from(e)),
    }
}