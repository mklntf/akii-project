use actix_web::{get, HttpResponse, Responder};
use log::info;

#[utoipa::path(get, path = "/api/tasks")]
#[get("/api/tasks")]
async fn list_tasks() -> impl Responder {
    info!("/api/tasks endpoint was called");
    HttpResponse::Ok().body("Task List")
}
