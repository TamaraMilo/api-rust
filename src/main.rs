

use actix_web::{delete, post, put, get, web, App, HttpResponse, HttpServer, HttpRequest};
mod errors;
mod file;
mod bucket;
mod user;
mod context;
mod auth;
mod responses;
use auth::UserClaims;
use context::EnvData;
use file::FileInfo;
use sea_orm::{DatabaseConnection, ActiveModelTrait, Set, EntityTrait};
use migration::{Migrator, MigratorTrait};
use actix_web::middleware::{Logger};
use entity::info::{ ActiveModel, Model};
use entity::info:: Model as File;
use entity::info::Entity as Info;
use entity::user::{ActiveModel as ActiveModelUser};
use entity::user::Role;
use entity::user:: Model as User;
use entity::user::Entity as UserEntity;
use entity::bucket::Entity as BucketEntity;
use entity::bucket::Model as BucketModel;
use entity::bucket::ActiveModel as ActiveModelBucket;
use actix_easy_multipart::extractor::{MultipartForm,MultipartFormConfig};
use dotenv::dotenv;
use bucket::Bucket;
use actix_jwt_auth_middleware::{Authority, AuthService};
use pwhash::bcrypt;
use validator::Validate;
use crate::auth::{LoginRequest, SingInRequest, validate, UserIdetifier};
use crate::context::AppState;
use crate::errors::{Errors};
use crate::file::UploadData;
use crate::responses::{ResponseText, LoginResponse, FileDetailsResponse};
use crate::user::{add_user, user_exist};

#[macro_use]
extern crate lazy_static;
mod repository;




async fn verify_service_request(user_claims: UserClaims) ->bool{
    match user_claims.role {
        Role::Admin => true,
        Role::User=>true,
        _=>false,
    }
}
#[post("/logout")]
async fn logout(req: HttpRequest) -> Result<HttpResponse, Errors> {   
    let mut cookie = match req.cookie("auth-token") {
    Some(r)=>r,
    None=> return Err(Errors::LoggedOut),
    };
    cookie.make_removal();
    Ok(HttpResponse::Ok().cookie(cookie).json(ResponseText::LoggedOut.to_string()))
}

#[get("/login")] 
async fn login( data: web::Data<AppState>, user: web::Json<LoginRequest>) -> Result<HttpResponse, Errors> {

    let conn=&data.conn;

    let user_db = match user_exist(conn, UserIdetifier{email: user.identifier.to_string(), username: user.identifier.to_string()}).await {
        Ok(result) => result,
        Err(_) => return Err(Errors::DatabaseError)
    };
    let user_db = match user_db{
        Some(user) => user,
        None => return Err(Errors::NoUser)
    };


    if !bcrypt::verify(user.password.to_string(), &user_db.password) {
        return Err(Errors::IncorrectUserPassword)
    }

    
    let mut cookie =  match data.auth.create_signed_cookie(UserClaims{
        id: user_db.user_id.to_string(),
        role: user_db.role.clone(),
    }){
        Ok(r)=>r,
        Err(_) => return Err(Errors::CreatingCookieError)
    };

    cookie.set_secure(false);
    

    Ok(HttpResponse::Accepted().cookie(cookie).json( LoginResponse {
            user_id: user_db.user_id,
            username: user_db.username,
            email: user_db.email,
            role: user_db.role,
        }))
}
#[post("/sing-in")]
async fn sign_in(data: web::Data<AppState>, user: web::Json<SingInRequest>) -> Result<HttpResponse, Errors> {
    let conn = &data.conn;
    match user.validate() {
        Ok(_)=>(),
        Err(_) => return Err(Errors::PassAndUsernameError)
    };

    let user_exist = match user_exist(conn, UserIdetifier{email: user.email.to_string(), username: user.username.to_string()}).await {
        Ok(result) => result,
        Err(_)=>return Err(Errors::DatabaseError)
    };

    if user_exist.is_some(){
        return Err(Errors::PassAndUsernameError)
    }

    match add_user(conn, user.0).await {
        Ok(result) => return Ok(HttpResponse::Ok().json(result)),
        Err(_) => return Err(Errors::DatabaseError)
    };



}
#[put("/users/{user_id}/role")]
async fn user_admin(data: web::Data<AppState>, user_id: web::Path<String>,user_claims:UserClaims) -> Result<HttpResponse, Errors> {
    if user_claims.role != Role::Admin {
        return Err(Errors::Unauthorized)
    }
    
    let conn = &data.conn;

    let user = match UserEntity::find_by_id(user_id.to_string()).one(conn).await {
        Ok(r)=> r,
        Err(_)=> return Err(Errors::DatabaseError)
    };
    let user = match user {
        Some(r)=> r,
        None=> return Err(Errors::DatabaseError)
    };


    let mut user: ActiveModelUser = user.into();
    user.role = Set(Role::Admin);
    match user.update(conn).await {
        Ok(user)=>return Ok(HttpResponse::Ok().json( LoginResponse {
                user_id: user.user_id,
                username: user.username,
                email: user.email,
                role: user.role
            })),
        Err(_)=> return Err(Errors::DatabaseError)
    };

}
#[get("/files/{id}")]
async fn get_file(data: web::Data<AppState>,id: web::Path<String>,user_claims:UserClaims) -> Result<HttpResponse, Errors>{
    
    let conn = &data.conn;
    let file_info = match Info::find_by_id(id.to_string()).one(conn).await {
        Ok(r)=> r,
        Err(_) => return Err(Errors::DatabaseError)
    };


   let file_info = match file_info {
        Some(r ) => r,
        None=> return Err(Errors::NoFileError)
        
    };
    
    if !validate(file_info.user_id.to_string(),user_claims.id,user_claims.role) {
        return Err(Errors::Unauthorized)
    }
    Ok(HttpResponse::Ok().json(file_info))

}
#[post("/files/{bucketId}")]
async fn save_file_in_bucket(data: web::Data<AppState>,bucket_id: web::Path<String>, mut payload: MultipartForm<UploadData>,user_claims: UserClaims) -> Result<HttpResponse,Errors> {
   
    let conn = &data.conn;

    let bucket = match BucketEntity::find_by_id(bucket_id.to_string()).one(conn).await{
        Ok(r)=> r,
        Err(_) => return Err(Errors::DatabaseError)
    };

    let bucket = match bucket{
        Some(r)=> r,
        None => return Err(Errors::NoBucketError)
    };
     
   
    if !validate(bucket.user_id.to_string(), user_claims.id.to_string(), user_claims.role) {
        return Err(Errors::Unauthorized)
    }

    
    let file_info = match FileInfo::new(&mut payload.image,bucket_id.to_string(),&data.env_data.basic_storage) {
        Ok(r)=>r,
        Err(_)=> return Err(Errors::BucketNotExisting)
    };
    
    let file = ActiveModel {
        id: Set(file_info.id.to_string()),
        extension: Set(file_info.extension.to_string()),
        path: Set(file_info.path.to_string()),
        user_id: Set(user_claims.id.to_string())
    };
    match file.insert(conn).await {
        Ok(_)=>return  Ok(HttpResponse::Ok().json( Model {
                    id: file_info.id.to_string(),
                    extension: file_info.extension,
                    path: file_info.path,
                    user_id: user_claims.id
                })),
        Err(_)=> return Err(Errors::DatabaseError)
    };
   
}
#[put("/files/{id}")]
async fn change_file(data: web::Data<AppState>,id: web::Path<String>,mut payload: MultipartForm<UploadData>, user_claims: UserClaims) -> Result<HttpResponse, Errors> {
    
    let conn = &data.conn;

    let file = match Info::find_by_id(id.to_string()).one(conn).await{
        Ok(r)=>r,
        Err(_)=>return Err(Errors::DatabaseError)
    };
    
  
    let file  = match file {
        Some(r) => r,
        None=> return Err(Errors::NoFileError)
    };
   
    if !validate(file.user_id.to_string(), user_claims.id.to_string(), user_claims.role) {
        return Err(Errors::Unauthorized)
    }
    let new_file_info =match FileInfo::change_data(file::ChangeArgs{data:  &mut payload.image,file_info: &file}) {
        Ok(r)=>r,
        Err(_) =>return Err(Errors::ChangeFileError)
    };

    let mut file: entity::info::ActiveModel = file.into();
    file.path = Set(new_file_info.path.clone());
    file.extension = Set(new_file_info.extension.to_string());
    match file.update(conn).await{
        Ok(_)=> return Ok(HttpResponse::Ok().json( FileDetailsResponse{
                id: id.to_string(),
                path: new_file_info.path,
                extension: new_file_info.extension .to_string(),
                })),
        Err(_) => return Err(Errors::DatabaseError)
    };

    
    
}
#[delete("/files/{id}")]
async fn delete_file(data: web::Data<AppState>,id: web::Path<String>, user_claims: UserClaims) -> Result<HttpResponse, Errors> {
   
    let conn = &data.conn;

    let file = Info::find_by_id(id.to_string()).one(conn).await.unwrap();
    
    let file = match file {
        Some(r) => r,
        None=> return Err(Errors::NoFileError)
    };
    if !validate(file.user_id.to_string(), user_claims.id.to_string(), user_claims.role) {
        return Err(Errors::Unauthorized)
    }
    match FileInfo::delete( file.path) {
        Ok(_) => (),
        Err(_) => return Err(Errors::DeletingFileError)
    };
    match Info::delete_by_id(id.to_string()).exec(conn).await {
        Ok(_) =>return  Ok(HttpResponse::Ok().finish()),
           
        Err(_)=>return Err(Errors::DeletingFileError)
    };
    
}
#[post("/bucket")]
async fn new_bucket(data: web::Data<AppState>,user_claims:UserClaims) -> Result<HttpResponse,Errors> {
    
    let conn = &data.conn;

    let bucket_id = match Bucket::new(&data.env_data.basic_storage) {
        Ok(r) => r,
        Err(_)=> return Err(Errors::BucketCreateError)
    };
    let bucket = ActiveModelBucket{
        bucket_id: Set(bucket_id.id.to_string()),
        user_id: Set(user_claims.id.to_string())
    };
    match bucket.insert(conn).await {
        Ok(r)=> return Ok(HttpResponse::Ok().json(r)),
        Err(_)=> return Err(Errors::DatabaseError)
        
    };
}
#[delete("/bucket/{bucket_id}")]
async fn delete_bucket(bucket_id: web::Path<String>,data: web::Data<AppState>,user_claims:UserClaims) -> Result<HttpResponse, Errors>{
    
    let conn = &data.conn;
    let  bucket = match BucketEntity::find_by_id(bucket_id.to_string()).one(conn).await{
        Ok(r)=> r,
        Err(_) => return Err(Errors::DatabaseError)
    };

    let bucket = match bucket{
        Some(r)=> r,
        None => return Err(Errors::NoBucketError)
    };
     
    if !validate(bucket.user_id.to_string(), user_claims.id.to_string(), user_claims.role) {
        return Err(Errors::Unauthorized)
    }
    match Bucket::delete(bucket_id.to_string(),&data.env_data.basic_storage) {
        Ok(())=> return Ok(HttpResponse::Ok().finish()),
        Err(_)=> return Err(Errors::BucketDeleteError)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    
    let env_data = EnvData::load().unwrap();
   
    let conn : DatabaseConnection = sea_orm::Database::connect( &env_data.database_url).await.expect("Error in conenction");

    let auth_authority = Authority::<UserClaims>::default();
    
    Migrator::up(&conn,None).await.expect("Error performing migrations");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState{ 
                auth: auth_authority.clone(),
                conn:conn.clone(),
                env_data: env_data.clone(),
            }))
            .service(sign_in)
            .service(logout)
            .service(login)
            .app_data(MultipartFormConfig::default().file_limit(env_data.max_transfer_size))
            .service(web::scope("")
            .service(change_file)
            .service(delete_file)
            .service(get_file)
            .service(new_bucket)
            .service(save_file_in_bucket)
            .service(user_admin)
            .service(delete_bucket)
            .wrap(AuthService::new(
                auth_authority.clone(),
                verify_service_request,
            )))
            .wrap(Logger::default())

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
