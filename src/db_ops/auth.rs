use crate::{
    db_ops::process_tokens, errors::AppError, models::{AuthorizationDatabase, NewUser, User}, routes::AuthenticationRequest, schema::{tokens, users}
};

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use diesel::prelude::*;
use process_tokens::create_token;

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

pub fn get_user_by_token(processed_token: String, conn: &mut PgConnection) -> Result<User, AppError>{
    match users::table
        .left_join(tokens::table)
        .select((users::id, users::username, users::access_rights, users::company, users::secret))
        .filter(tokens::token.eq(&processed_token))
        .get_result::<User>(conn)
        {   
            Ok(user) => Ok(user),
            Err(e) => return Err(AppError::from(e)),
        }
}