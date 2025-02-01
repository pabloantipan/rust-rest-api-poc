// src/main.rs
mod models;
mod routes;

use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use routes::{create_user, get_users, get_user, update_user, delete_user, AppState};
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app_state = web::Data::new(AppState {
        users: Mutex::new(Vec::new()),
    });

    println!("Starting server at {}", addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(
                web::scope("/api")
                    .route("/users", web::post().to(create_user))
                    .route("/users", web::get().to(get_users))
                    .route("/users/{id}", web::get().to(get_user))
                    .route("/users/{id}", web::put().to(update_user))
                    .route("/users/{id}", web::delete().to(delete_user))
            )
    })
    .bind(addr)?
    .run()
    .await
}