use crate::{
    errors::AppError, schema::{timers, tokens, users, jobs},
};
use diesel::{backend::Backend, deserialize::{self, FromSql, FromSqlRow}, serialize::ToSql, sql_types::Integer};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub access_rights: AccessRights,
    pub company: Option<String>,
    pub secret: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub secret: &'a str,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = timers)]
pub struct Timer<'a> {
    pub user_id: &'a Uuid,
    pub job: &'a str,
    pub started_at: &'a str,
    pub finished_at: &'a str,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = tokens)]
pub struct Token<'a> {
    pub token: &'a str,
    pub user_id: &'a Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = jobs)]
pub struct NewJob<'a> {
    pub job: &'a str,
    pub user_id: &'a Uuid,
}

#[derive(PartialEq, Debug, FromSqlRow, Clone, Copy)]
//#[diesel(sql_type = Integer)]
pub enum AccessRights {
    Admin,
    User, 
    Unregistered, 
}

impl<DB> diesel::deserialize::FromSql<Integer, DB> for AccessRights
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(AccessRights::Admin),
            1 => Ok(AccessRights::User),
            2 => Ok(AccessRights::Unregistered),
            x => Err(format!("Unrecognized variant in user rights {}", x).into())
        }
    }
}

impl<DB> diesel::serialize::ToSql<Integer, DB> for AccessRights
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, DB>) -> diesel::serialize::Result {
        match self {
            AccessRights::Admin => 0.to_sql(out),
            AccessRights::User => 1.to_sql(out),
            AccessRights::Unregistered => 2.to_sql(out),
        }
    }
}

pub trait AuthorizationDatabase {
    fn login(&self, conn: &mut PgConnection) -> Result<String, AppError>;
    fn registration(&self, conn: &mut PgConnection) -> Result<(), AppError>;
}