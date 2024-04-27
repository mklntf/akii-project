use actix_web::{get, HttpResponse, Responder};

#[utoipa::path(get, path = "/api/tasks")]
#[get("/api/tasks")]
async fn list_tasks() -> impl Responder {
    HttpResponse::Ok().body("Task List")
}
