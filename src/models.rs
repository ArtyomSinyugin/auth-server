pub mod process_tokens;
pub mod job;

use crate::{
    errors::AppError, routes::AuthenticationRequest, schema::{timers, tokens, users, jobs}, AccessRights
};
use diesel::{backend::Backend, deserialize::{self, FromSql}, serialize::ToSql, sql_types::Integer};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use diesel::prelude::*;
use process_tokens::create_token;
use uuid::Uuid;

#[derive(Queryable, Debug,  PartialEq)]
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
pub struct NewTimer<'a> {
    pub user_id: &'a Uuid,
    pub job: &'a str,
    pub started_at: &'a str,
    pub finished_at: &'a str,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = tokens)]
pub struct NewToken<'a> {
    pub token: &'a str,
    pub user_id: &'a Uuid,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = jobs)]
pub struct NewJob<'a> {
    pub job: &'a str,
    pub user_id: &'a Uuid,
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

impl AuthorizationDatabase for AuthenticationRequest {
    fn login(&self, conn: &mut PgConnection) -> Result<String, AppError> {
        dbg!(&self.login);
        dbg!(&self.password);
        match users::table
            .filter(users::username.eq(self.login.to_lowercase()))
            .get_result::<User>(conn)
        {
            Ok(user) => {
                let parsed_hash = PasswordHash::new(&user.secret)?;
                if Argon2::default()
                    .verify_password(self.password.as_bytes(), &parsed_hash)
                    .is_ok()
                {
                    // Создаём токен, который вернётся в routes!!!
                    create_token(user, conn)
                } else {
                    Err(AppError::WrongPassword)
                }
            },
            Err(e) => Err(AppError::from(e)),
        }
    }

    fn registration(&self, conn: &mut PgConnection) -> Result<(), AppError> {
        let salt = match self.password.len() {
            n if n < 8 => return Err(AppError::WeakPassword),
            n if n > 128 => return Err(AppError::TooLongPassword),
            _ => SaltString::generate(&mut OsRng),
        };

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(self.password.as_bytes(), &salt)?
            .to_string();

        let new_user = NewUser {
            username: &self.login.to_lowercase(),
            secret: &password_hash,
        };

        return match diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(conn)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::from(e)),
        };
    }
}