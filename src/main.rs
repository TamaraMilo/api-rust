use crate::auth::auth_controller::{singin, singup};
use crate::auth::jwt_service::validator;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use bucket::bucket_controller::{delete_bucket, new_bucket};
use context::{AppState, EnvData};
use dotenv::dotenv;
use file::file_controller::{change_file, create_file, delete_file, get_file, get_files_page, show_file_url};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use user::user_controller::user_admin;
mod auth;
mod bucket;
mod context;
mod errors;
mod file;
mod response_dto;
mod repository;

mod user;
#[macro_use]
extern crate lazy_static;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let env_data = EnvData::load().unwrap();

    let conn: DatabaseConnection = sea_orm::Database::connect(&env_data.database_url)
        .await
        .expect("Error in conenction");

    Migrator::up(&conn, None)
        .await
        .expect("Error performing migrations");

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .app_data(web::Data::new(AppState {
                conn: conn.clone(),
                env_data: env_data.clone(),
            }))
            .wrap(Logger::default())
            .service(show_file_url)
            .service(
                web::scope("/auth")
                    .service(singup)
                    .service(singin),
            )
            .service(
                web::scope("")
                    
                    .wrap(auth)
                    .service(
                        web::scope("file")
                            .service(change_file)
                            .service(delete_file)
                            .service(get_file)
                            .service(create_file)
                            .service(get_files_page),
                    )
                    .service(
                        web::scope("bucket")
                            .service(new_bucket)
                            .service(delete_bucket),
                    )
                    .service(web::scope("user").service(user_admin)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    // App::new().service(web::scope("/api")).service(user_controller)
}
