#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use log::info;
use std::env;

pub mod models;
pub mod schema;
pub mod unit;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn ingredient(db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    use schema::ingredients::dsl::*;

    info!("hello");
    let res: Vec<models::Ingredient> = web::block(move || {
        let conn = db.get().unwrap();

        ingredients.limit(5).load::<models::Ingredient>(&conn)
    })
    .await?;

    Ok(HttpResponse::Ok().json(res))
}

async fn insert_ingredient(
    db: web::Data<DbPool>,
    new_ingredient: web::Json<models::NewIngredient>,
) -> Result<HttpResponse, Error> {
    use schema::ingredients;
    info!("{:?}", &new_ingredient);
    let res: models::Ingredient = web::block(move || {
        let conn = db.get().unwrap();

        diesel::insert_into(ingredients::table)
            .values(&new_ingredient.into_inner())
            .get_result(&conn)
    })
    .await?;

    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_path = env::var("DATABASE_URL").expect("Error while retrieving env variable for db");
    let manager = ConnectionManager::<PgConnection>::new(db_path);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create the pool");

    env_logger::init();

    info!("Starting server");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/ingredient").service(
                    web::resource("/")
                        .app_data(web::JsonConfig::default().limit(4096))
                        .route(web::get().to(ingredient))
                        .route(web::post().to(insert_ingredient)),
                ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
