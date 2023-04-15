// #[macro_use]
// extern crate diesel;

// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use diesel::prelude::*;
// use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
// use r2d2::Pool;
use std::env;

// pub type PostgressPool = Pool<ConnectionManager<PgConnection>>;

// pub fn get_pool() -> PostgressPool {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
//     let manager = ConnectionManager::<PgConnection>::new(database_url);

//     r2d2::Pool::builder()
//         .build(manager)
//         .expect("could not build connection pool")
// }

// #[get("/student/{class_id}/current_slide")]
// async fn student_current_slide(
//     pool: web::Data<PostgressPool>,
//     path: web::Path<u32>,
// ) -> impl Responder {
//     let class_id = path.into_inner();
//     HttpResponse::Ok().body(format!("Class {}", class_id))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let pool = get_pool();

//     HttpServer::new(move || {
//         App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .service(student_current_slide)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

#[macro_use]
extern crate diesel;

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod handlers;
mod models;
mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let port = std::env::var("PORT").expect("$PORT is not set.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .service(handlers::classes_index)
            .service(handlers::classes_create)
            .service(handlers::classes_show)
            .service(handlers::classes_update)
            .service(handlers::classes_destroy)
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}
