use crate::handlers::*;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/drinks")
            .route("", web::get().to(list_drinks))
            .route("", web::post().to(create_drink))
            .route("/search", web::get().to(search_drinks))
            .route("/brand_summary", web::get().to(brand_summary))
            .route("/{id}", web::get().to(get_drink))
            .route("/{id}", web::put().to(update_drink))
            .route("/{id}", web::delete().to(delete_drink)),
    )
    .route("/", web::get().to(hello))
    .route("/echo", web::post().to(echo));
}
