use crate::auth::auth_controller::{singin, singout, singup};
use crate::auth::dto::UserClaims;
use crate::auth::jwt_service::validator;
use actix_jwt_auth_middleware::{AuthService, Authority};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use bucket::bucket_controller::{delete_bucket, new_bucket};
use context::{AppState, EnvData};
use dotenv::dotenv;
use file::file_controller::{change_file, create_file, delete_file, get_file};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use user::user_controller::user_admin;
mod auth;
mod bucket;
mod context;
mod errors;
mod file;
mod repository;
mod responses;
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

    // let auth_authority = Authority::<UserClaims>::default();

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
            .service(
                web::scope("/auth")
                    .service(singup)
                    .service(singout)
                    .service(singin),
            )
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .service(change_file)
                    .service(delete_file)
                    .service(get_file)
                    .service(create_file)
                    .service(new_bucket)
                    .service(delete_bucket)
                    .service(user_admin),
            )
            
    })
    // App::new()
    //     .app_data(web::Data::new(AppState {
    //         auth: auth_authority.clone(),
    //         conn: conn.clone(),
    //         env_data: env_data.clone(),
    //     }))
    //     .app_data(MultipartFormConfig::default().file_limit(env_data.max_transfer_size))
    //     .service(web::scope("auth")
    //             .service(singup)
    //             .service(singout)
    //             .service(singin))
    //
    //     .service(web::scope("bucket")
    //             .service(new_bucket)
    //             .service(delete_bucket)
    //             .wrap(AuthService::new(
    //                 auth_authority.clone(),
    //                 guard,
    //     )))
    //     .service(web::scope("user")
    //             .service(user_admin)
    //             .wrap(AuthService::new(
    //                 auth_authority.clone(),
    //                 guard_admin,
    //     )))
    //     .wrap(Logger::default())
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    // App::new().service(web::scope("/api")).service(user_controller)
}
