pub mod db_ops;
pub mod configuration;
mod errors;
mod middleware;
mod models;
mod routes;
mod schema;

use crate::{
    db_ops::*, 
    models::AccessRights,
    };
use actix_web::{web, App, HttpServer};
use configuration::Configuration;

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
    config: Configuration,
}

// std::cell::Cell<Uuid> - для изменяемости в структуре
impl AuthServer {
    pub fn new(port: u16, config: Configuration) -> Self {
        AuthServer { port, config }
    }

    pub async fn run(self, database_url: String) -> std::io::Result<()> {
        // создаём пул подключений к базе данных
        let pool = manage_database(database_url);
        let ip = self.config.ip.clone();
        let user = web::Data::new(AuthorizedUser {
            user_id: Mutex::new(Uuid::nil()),
            user_name: Mutex::new("".to_string()),
            access_rights: Mutex::new(AccessRights::Unregistered),
        });

        run_migrations(&mut pool.get().unwrap())
            .map_err(|e| eprint!("Автоматизация миграций провалилась, причина: {:?}", e))
            .unwrap();
        println!("Сервер запущен по адресу http://{}:{}", ip, self.config.port);
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .app_data(user.clone())
                .app_data(web::Data::new(self.config.clone()))
                .wrap(middleware::Authorization)
                .service(web::scope("/api-v1").guard(AccessRights::guard(AccessRights::Unregistered)).configure(routes::config_authentification))
                .default_service(actix_files::Files::new("/", "../timer/dist/").index_file("index.html"))
        })
        .bind((ip, self.port))?
        .run()
        .await
    }
}