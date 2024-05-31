use std::env;

use actix_web::{web, App, HttpServer};
use handlers::TaskCreateInput;
use log::{info, LevelFilter};
use model::Task;
use service::Service;
use std::io::Result;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod database;
mod handlers;
mod model;
mod repository;
mod service;

struct AppState {
    service: Service,
}

#[actix_web::main]
async fn main() -> Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            handlers::list_tasks,
            handlers::create_task,
            handlers::get_task,
            handlers::delete_task
        ),
        components(schemas(Task, TaskCreateInput))
    )]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();

    let port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT must be a valid port number");

    let db = env::var("DATABASE_URL").unwrap_or_else(|_| "/tmp/tasks.db".to_string());

    let pool = database::create_pool(db.as_ref());
    let repository = repository::Database::new(pool);
    let service = Service::new(Box::new(repository));
    let app_state = web::Data::new(Arc::new(AppState { service }));

    env_logger::builder().filter_level(LevelFilter::Info).init();

    info!("Starting server on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                SwaggerUi::new("/api/swagger-ui/{_:.*}").url("/api/openapi.json", openapi.clone()),
            )
            .service(web::redirect("/", "/api/swagger-ui/"))
            .service(handlers::list_tasks)
            .service(handlers::create_task)
            .service(handlers::get_task)
            .service(handlers::delete_task)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
