mod db;
mod handlers;
mod models;
mod routes;

use crate::db::get_db_pool;
use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = get_db_pool().await;

    let bind_address = dotenvy::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:8181".to_string());

    println!("Starting server at http://{}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
            .wrap(Logger::new(
                r#"%a "%r" %s-code %b-bytes "%{Referer}i" "%{User-Agent}i" %T-sec"#,
            ))
            .wrap(
                Cors::default()
                .allowed_origin("http://localhost:5511")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec!["Content-Type"])
                .max_age(3600)
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind(bind_address)?
    .run()
    .await
}
