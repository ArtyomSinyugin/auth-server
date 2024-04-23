use crate::{
    errors::AppError, AccessRights, routes::AuthenticationRequest, schema::{tokens, users}
};
use diesel::{backend::{self, Backend}, deserialize::{self, FromSql}, serialize::ToSql, sql_types::Integer};
use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        SaltString,
    },
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub access_rights: AccessRights,
    pub secret: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub secret: &'a str,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = tokens)]
pub struct NewToken<'a> {
    pub token: &'a str,
    pub user_id: &'a Uuid,
}

impl<DB> diesel::deserialize::FromSql<Integer, DB> for AccessRights
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: backend::RawValue<DB>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            1 => Ok(AccessRights::Admin),
            2 => Ok(AccessRights::User),
            3 => Ok(AccessRights::Unregistered),
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
            }
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

pub fn create_token(user: User, conn: &mut PgConnection) -> Result<String, AppError> {
    let mut token_bytes = [0u8, 32];
    OsRng.fill_bytes(&mut token_bytes);
    let token_string = base64::encode(token_bytes);
    let token_entry = NewToken {
        token: &token_string,
        user_id: &user.id,
    };
    match diesel::insert_into(tokens::table)
        .values(token_entry)
        .execute(conn)
    {
        Ok(_) => Ok(token_string),
        Err(e) => Err(AppError::from(e)),
    }
}

pub fn check_token(processed_token: String, conn: &mut PgConnection) -> Result<User, AppError> {
    match users::table
        .left_join(tokens::table.on(tokens::user_id.eq(users::id)))
        .select((users::id, users::username, users::access_rights, users::secret))
        .filter(tokens::token.eq(&processed_token))
        .get_result::<User>(conn)
    {
        Ok(user) => Ok(user),
        Err(_) => Err(AppError::UnauthorizedUser),
    }
}
