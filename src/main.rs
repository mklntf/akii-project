use std::env;

use actix_web::{web, App, HttpServer};
use handlers::TaskCreateInput;
use log::{info, LevelFilter};
use service::{Service, Task};
use std::io::Result;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
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

    env_logger::builder().filter_level(LevelFilter::Info).init();

    info!("Starting server on port {}", port);

    let service = Service::new();
    let app_state = web::Data::new(Arc::new(AppState { service }));

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
