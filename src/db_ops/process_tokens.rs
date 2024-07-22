use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use argon2::password_hash::rand_core::{OsRng, RngCore};
use crate::{
    db_ops::PgConnection, errors::AppError, models::{NewToken, Token, User}, schema::tokens
};

pub fn create_token(user: User, conn: &mut PgConnection) -> Result<String, AppError> {
    let mut token_bytes = [0u8; 32];
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

pub fn fetch_token(processed_token: String, conn: &mut PgConnection) -> Result<Token, AppError> {
    match tokens::table.select(Token::as_select()).filter(tokens::token.eq(processed_token)).get_result::<Token>(conn) {  // попробовать вместо get_user_by_token использовать связь belong to
        Ok(token) => Ok(token),
        Err(e) => Err(AppError::from(e)),
    }
}