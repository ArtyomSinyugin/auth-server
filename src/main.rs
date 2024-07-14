// http://192.160.21.125:8000/

use auth_server::AuthServer;
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT не спарсился в u16");

    let database_url = env::var("DATABASE_URL").expect("Переменная среды DATABASE_URL не найдена");

    let app = AuthServer::new(port);

    app.run(database_url).await
}
