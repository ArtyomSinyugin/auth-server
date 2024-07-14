use std::error::Error;

use diesel::backend::Backend;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn manage_database(database_url: String) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Пул подключений к БД не создан")
}

pub fn run_migrations<DB: Backend>(
    pool: &mut impl MigrationHarness<DB>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    pool.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
