use std::env;

use actix_web::{web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(paths(handlers::list_tasks), components(schemas()))]
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
            .service(handlers::list_tasks)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
