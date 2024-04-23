mod db_connection;
mod errors;
mod middleware;
mod models;
mod routes;
mod schema;

use crate::{db_connection::*, routes::{characters, for_staff, page, guards::AccessRights}};
use actix_web::{web, App, HttpServer};

use std::{cell::Cell, sync::{Arc, Mutex}};
use uuid::Uuid;

pub struct AuthorizedUser {
    pub user_id: Cell<Uuid>,
    pub user_name: Arc<Mutex<String>>, 
    pub access_rights: Arc<Mutex<AccessRights>>,
}

pub struct AuthServer {
    port: u16,
}

// std::cell::Cell<Uuid> - для изменяемости в структуре
impl AuthServer {
    pub fn new(port: u16) -> Self {
        AuthServer { port }
    }

    pub async fn run(self, database_url: String) -> std::io::Result<()> {
        // создаём пул подключений к базе данных
        let pool = manage_database(database_url);

        // автоматизируем миграции
        run_migrations(&mut pool.get().unwrap())
            .map_err(|e| eprint!("Автоматизация миграций провалилась, причина: {:?}", e))
            .unwrap();
        // запускаем сервер
        println!("Сервер запущен");
        HttpServer::new(move || {
            App::new()
                // деаем мст к азе данных.
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(AuthorizedUser {
                    user_id: Cell::new(Uuid::nil()),
                    user_name: Arc::new(Mutex::new("".to_string())),
                    access_rights: Arc::new(Mutex::new(AccessRights::Unregistered)),
                }))
                // create null uuid for user (init AuthorizedUser struct)
                .wrap(middleware::Authorization)
                .service(web::scope("/api-v1").service(page))
                .service(web::scope("/api-v1").guard(AccessRights::guard(AccessRights::Unregistered)).configure(routes::config_authentification))
                .service(web::scope("/api-v1").guard(AccessRights::guard(AccessRights::User)).service(characters))
                .service(web::scope("/api-v1").guard(AccessRights::guard(AccessRights::Admin)).service(for_staff))
        })
        .bind(("127.0.0.1", self.port))?
        .run()
        .await
    }
}
