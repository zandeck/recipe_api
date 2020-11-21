use actix_web::{get, middleware, web, App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_path = env::var("DATABASE_URL").expect("Error while retrieving env variable for db");
    let manager = ConnectionManager::<PgConnection>::new(db_path);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create the pool");

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
