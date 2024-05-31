use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::model::Task;
use crate::AppState;

#[derive(Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TaskCreateInput {
    name: String,
}

#[utoipa::path(
    get,
    path = "/api/tasks",
    tag = "Task API",
    responses(
        (
            status = 200, body = Vec<Task>, content_type = "application/json",
            example = json!([Task::new(0, "Example Task".to_string())])
        ),
    )
)]
#[get("/api/tasks")]
async fn list_tasks(state: web::Data<Arc<AppState>>) -> impl Responder {
    let tasks = state.service.list_tasks();

    info!("GET request on /api/tasks endpoint");

    HttpResponse::Ok()
        .content_type("applicationU/json")
        .body(serde_json::to_string(&tasks).unwrap())
}

#[utoipa::path(
    post,
    path = "/api/tasks",
    tag = "Task API",
    request_body = TaskCreateInput,
    responses(
        (
            status = 200, body = Task, content_type = "application/json",
            example = json!(Task::new(0, "Example Task".to_string()))
        ),
    )
)]
#[post("/api/tasks")]
async fn create_task(
    state: web::Data<Arc<AppState>>,
    input: web::Json<TaskCreateInput>,
) -> impl Responder {
    let task = state.service.create_task(input.name.clone());

    info!("POST request on /api/tasks endpoint");

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&task).unwrap())
}

#[utoipa::path(
    get,
    path = "/api/tasks/{id}",
    tag = "Task API",
    params(
        ("id" = u32, Path, description = "Task ID")
    ),
    responses(
        (
            status = 200, body = Task, content_type = "application/json",
            example = json!(Task::new(0, "Example Task".to_string()))
        ),
        (status = 404, description = "Task not found")
    )
)]
#[get("/api/tasks/{id}")]
async fn get_task(state: web::Data<Arc<AppState>>, id: web::Path<u32>) -> impl Responder {
    let task = state.service.get_task(*id);

    info!("GET request on /api/tasks/{id} endpoint");

    match task {
        Some(task) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&task).unwrap()),
        None => HttpResponse::NotFound().finish(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/tasks/{id}",
    tag = "Task API",
    params(
        ("id" = u32, Path, description = "Task ID")
    ),
    responses(
        (
            status = 200, body = Task, content_type = "application/json",
            example = json!(Task::new(0, "Example Task".to_string()))
        ),
        (status = 404, description = "Task not found")
    )
)]
#[delete("/api/tasks/{id}")]
async fn delete_task(state: web::Data<Arc<AppState>>, id: web::Path<u32>) -> impl Responder {
    let task = state.service.delete_task(*id);

    info!("DELETE request on /api/tasks/{id} endpoint");

    match task {
        Some(task) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&task).unwrap()),
        None => HttpResponse::NotFound().finish(),
    }
}
