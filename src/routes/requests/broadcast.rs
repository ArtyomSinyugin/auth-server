use actix_web::{get, web, Responder};

use crate::broadcast::Broadcaster;

#[get("/events")]
async fn event_stream(broadcaster: web::Data<Broadcaster>) -> impl Responder {
    broadcaster.new_client().await
}