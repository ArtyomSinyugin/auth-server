mod db_connection;
mod errors;
mod models;
mod routes;
mod schema;

use std::cell::Cell;

use actix_web::{HttpServer, App, web, dev::Service};
use uuid::Uuid;
use crate::{db_connection::*, routes::guards::{process_token, extract_header_token}};
use crate::routes::page;

pub struct AuthServer {
    port: u16,
}

pub struct AuthorizedUser {
    pub user_id: Cell<Uuid>,
}
// std::cell::Cell<Uuid> - для изменяемости в структуре
impl AuthServer {
    pub fn new (port: u16) -> Self {
        AuthServer { port }
    }

    pub async fn run (self, database_url: String)-> std::io::Result<()> {
// создаём пул подключений к базе данных
        let pool = manage_database(database_url);

// автоматизируем миграции 
        run_migrations(&mut pool
            .get()
            .unwrap()).map_err(|e| 
                eprint!("Автоматизация миграций провалилась, причина: {:?}", e))
            .unwrap();
// запускаем сервер   
        println!("Сервер запущен");
        HttpServer::new( move || {
            App::new()
                // деаем мст к азе данных. 
                .app_data(web::Data::new(pool.clone()))
                // create null uuid for user (init AuthorizedUser struct)
                .app_data(web::Data::new(AuthorizedUser { user_id: Cell::new(Uuid::nil()) }))
                .wrap_fn( |req, srv|{ // стр. 119 и никакого толка // со стр. 190 начинается что-то важное, стр. 211
                    
                    let processed_token = extract_header_token(&req).map_err(|e| println!("Ошибка с токеном: {e}")).unwrap();
                    
                    let pool = req.app_data::<web::Data<PgPool>>()
                        .expect("Ошибка подключения к базе данных в middleware").clone(); 

                    let auth_user = req.app_data::<web::Data<AuthorizedUser>>()
                        .expect("Не удалось извлечь данные AuthorizedUser").clone();

                    let fut = srv.call(req);
                    async {            
                        process_token(processed_token, pool, auth_user).await;             
                        let result = fut.await?;
                        Ok(result)
                    }
                })
// вот здесь middleware, если у нас поступает токен в виде json, то создаётся заголовок "login". Далее guard обрабатывает этот заголовок и всё впорядке. 
// если поступает ошибка из функции with_token, то перекидывает на необходимость залогиниться. 

                .service(web::scope("/api-v1").configure(routes::config_authentification))
                .service(web::scope("/api-v1").service(page))
            })
            .bind(("127.0.0.1", self.port))?   
            .run()
            .await
    }
}

/*
                    if *&req.path().contains("/item/") {
                        match routes::guards::process_token(&req, pool.clone()).await {
                            Ok(_token) => println!("the token is passable"),
                            Err(message) => println!("token error: {}", message),
                        }
                    }


                        match process_token(processed_token, pool.clone()).await {
                            Ok(token) => todo!(),
                            Err(e) => println!("token error: {}", e),
                        }


                        if process_token(processed_token, pool).await.expect("not true") {
                            result.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("Authorized"));
                            println!{"проходим барьер"};    
                        } else {
                            result.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("Unauthorized"));
                            println!{"не проходим барьер"}; 
                        }
 */