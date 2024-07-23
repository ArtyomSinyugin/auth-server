// http://192.160.21.125:8000/

use timer_server::AuthServer;
use dotenvy::dotenv;
use std::{env, path::PathBuf};
use timer_server::configuration::deserialize_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = deserialize_config(&PathBuf::from("C:\\RUST\\timer-server\\"));
    dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or_else(|_| config.port.clone())
        .parse::<u16>()
        .expect("PORT не спарсился в u16");

    let database_url = env::var("DATABASE_URL").expect("Переменная среды DATABASE_URL не найдена");

    let app = AuthServer::new(port, config);

    app.run(database_url).await
}