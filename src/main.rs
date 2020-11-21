use actix_web::{get, middleware, web, App, HttpServer, Responder};
use std::env;

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let _db_path = env::var("DATABASE_URL").expect("Error while retrieving env variable for db");

    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
