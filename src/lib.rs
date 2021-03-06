#![deny(warnings)]

mod graphql;
use graphql::schema::{Context, schema, Schema};

use std::{env};

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, Error, HttpResponse, HttpServer};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

async fn graphiql_route() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphgl", None).await
}
async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphgl", None).await
}
async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let ip_address = (*req.connection_info()).remote_addr().unwrap_or("Unknown").to_string();
    let context = Context::new(ip_address);
    graphql_handler(&schema, &context, req, payload).await
}

#[actix_web::main]
pub async fn run(host: &'static str, port: &'static str) -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let server_address = format!("{}:{}", host, port);
    let cors_origin = format!("http://{}", server_address);

    let server = HttpServer::new(move || {
        App::new()
            .data(schema())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin(&cors_origin)
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(
                web::resource("/graphgl")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_route)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql_route)))
    });
    server.bind(server_address).unwrap().run().await
}
// now go to http://127.0.0.1:8080/playground or graphiql and execute
//{  apiVersion,  user(id: 2){id, name}}

