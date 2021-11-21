#[macro_use]
extern crate diesel;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use actix_files as fs;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod dao;
mod utils;
mod service;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: dao::models::Pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::secrets::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(86400) // one day in seconds
                    .secure(false), // this can only be true if you have https
            ))
            // limit the maximum amount of data that server will accept
            .data(web::JsonConfig::default().limit(4096))
            // static files
            .service(fs::Files::new("/static", "./src/static/").show_files_listing())
            // everything below is under '/api/' route
            .service(
              web::scope("/api")
              .service(
                web::resource("/invitation")
                  .route(web::post().to(service::invitation::http_invitation_handler::post_invitation)),
              )
              .service(
                web::resource("/register/{invitation_id}")
                  .route(web::post().to(service::registration::http_register_handler::register_user)),
              )
              .service(
                web::resource("/auth")
                  .route(web::post().to(service::authentication::http_auth_handler::login))
                  .route(web::delete().to(service::authentication::http_auth_handler::logout))
                  .route(web::get().to(service::authentication::http_auth_handler::get_me)),
              ),
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
