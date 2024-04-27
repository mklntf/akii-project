use std::env;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[utoipa::path(get, path = "/api/tasks")]
#[get("/api/tasks")]
async fn list_tasks() -> impl Responder {
    HttpResponse::Ok().body("Task List")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(paths(list_tasks), components(schemas()))]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();

    let port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT must be a valid port number");

    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/api/swagger-ui/{_:.*}").url("/api/openapi.json", openapi.clone()),
            )
            .service(web::redirect("/", "/api/swagger-ui/"))
            .service(list_tasks)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
