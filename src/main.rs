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
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind(bind_address)?
    .run()
    .await
}
