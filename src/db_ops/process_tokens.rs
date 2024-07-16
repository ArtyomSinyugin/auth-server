use diesel::RunQueryDsl;
use argon2::password_hash::rand_core::{OsRng, RngCore};
use crate::{
    db_ops::PgConnection, errors::AppError, models::{Token, User}, schema::tokens
};

pub fn create_token(user: User, conn: &mut PgConnection) -> Result<String, AppError> {
    let mut token_bytes = [0u8, 32];
    OsRng.fill_bytes(&mut token_bytes);
    let token_string = base64::encode(token_bytes);
    let token_entry = Token {
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