pub mod db_ops;
mod errors;
mod middleware;
mod models;
mod routes;
mod schema;

use crate::{
    db_ops::*, 
    routes::requests::{characters, for_staff, page},
    models::AccessRights,
    };
use actix_web::{guard, web, App, HttpServer};
use routes::page404;

use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub struct AuthorizedUser {
    pub user_id: Mutex<Uuid>,
    pub user_name: Mutex<String>, 
    pub access_rights: Mutex<AccessRights>,
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
        let user = web::Data::new(AuthorizedUser {
            user_id: Mutex::new(Uuid::nil()),
            user_name: Mutex::new("".to_string()),
            access_rights: Mutex::new(AccessRights::Unregistered),
        });

        run_migrations(&mut pool.get().unwrap())
            .map_err(|e| eprint!("Автоматизация миграций провалилась, причина: {:?}", e))
            .unwrap();
        println!("Сервер запущен");
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .app_data(user.clone())
                .wrap(middleware::Authorization)
                .service(page)
                .service(web::scope("/auth").guard(AccessRights::guard(AccessRights::Unregistered)).configure(routes::config_authentification))
                .service(web::scope("/users").guard(guard::Any(AccessRights::guard(AccessRights::User))
                        .or(AccessRights::guard(AccessRights::Admin))
                    ).service(characters))
                .service(web::scope("/admins").guard(AccessRights::guard(AccessRights::Admin)).service(for_staff))
                .default_service(web::to(page404))
        })
        .bind(("192.160.21.125", self.port))?
        .run()
        .await
    }
}
