
use actix_web::{delete, post, put, get, web, App, HttpResponse, HttpServer};
mod errors;
mod file;
use file::FileInfo;
use sea_orm::{DatabaseConnection, ActiveModelTrait, Set, EntityTrait};
use migration::{Migrator, MigratorTrait};
use actix_web::middleware::Logger;
use entity::info::{ ActiveModel, Model};
use entity::info::Entity as Info;
use errors::CustomError;
use actix_easy_multipart::{FromMultipart,File};
use actix_easy_multipart::extractor::{MultipartForm,MultipartFormConfig};
use serde::Serialize;
use dotenv::dotenv;



#[derive(Debug,Clone)]
struct AppState {
    conn: DatabaseConnection,
}
#[derive(FromMultipart,Debug)]
struct Upload {
   image: File,
}
#[derive(Serialize)]
struct Response<T>
{
    data: Option<T>,
    errors: Option<T>
}
#[derive(Serialize)]
struct FileDetails {
    id: String,
    extension:String,
    path:String,
}

#[get("/files/{id}")]
async fn get_file(data: web::Data<AppState>,id: web::Path<String>) -> HttpResponse{

    let conn = &data.conn;
    let _file =match Info::find_by_id(id.to_string()).one(conn).await.unwrap() {
        Some(r)=> return  HttpResponse::Ok().json(Response{
            data: Some(r),
            errors: None,
        }),
        None => return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(CustomError::NoFileError.error_response()),
        })
    };
   
}

#[post("/files")]
async fn save_file(data: web::Data<AppState>,mut payload: MultipartForm<Upload>) -> HttpResponse {

    let conn = &data.conn;
    
 
    let file_info = match FileInfo::new(&mut payload.image) {
        Ok(r) => r,
        Err(_) => return HttpResponse::BadRequest().json(Response{
                data: None,
                errors: Some(CustomError::WritingFileError.error_response())     
            })
    };
    let file = ActiveModel {
        id: Set(file_info.id.to_string()),
        extension: Set(file_info.extension.to_string()),
        path: Set(file_info.path.to_string())
    };


    let _file: Model = file.insert(conn).await.expect(&CustomError::WritingFileError.error_response());
    HttpResponse::Ok().json(
        Response{ 
            data: Some( Model {
                id: file_info.id.to_string(),
                extension: file_info.extension,
                path: file_info.path,
            }),
            errors: None
        })
    
}



#[put("/files/{id}")]
async fn change_file(data: web::Data<AppState>,id: web::Path<String>,mut payload: MultipartForm<Upload>) -> HttpResponse {

    let conn = &data.conn;

    let file = Info::find_by_id(id.to_string()).one(conn).await.unwrap();
    
    if file == None {
        return HttpResponse::BadRequest().json(Response {
            data: None,
            errors: Some(CustomError::NoFileError.error_response()),
        })
    }
    
    let new_file_info =match FileInfo::change_data(file::ChangeArgs{data:  &mut payload.image, path: file.as_ref().unwrap().path.clone()}) {
        Ok(r)=>r,
        Err(_) => return HttpResponse::BadRequest().json(Response {
            data: None,
            errors: Some(CustomError::ChangeFileError.error_response()),
        }),
    };

    let mut file: entity::info::ActiveModel = file.unwrap().into();
    file.path = Set(new_file_info.path.clone());
    file.extension = Set(new_file_info.extension.to_string());
    file.update(conn).await.unwrap();

    HttpResponse::Ok().json(Response {
        data: Some( Model {
            id: id.to_string(),
            path: new_file_info.path,
            extension: new_file_info.extension .to_string(),
        }),
        errors: None,
    })
    
}



#[delete("/files/{id}")]
async fn delete_file(data: web::Data<AppState>,id: web::Path<String>) -> HttpResponse {

    let conn = &data.conn;

    let file = Info::find_by_id(id.to_string()).one(conn).await.unwrap();
    
    if file.is_none() {
        return HttpResponse::BadRequest().json(Response {
            data: None,
            errors: Some(CustomError::NoFileError.error_response()),
        })
    }
    match FileInfo::delete( file.unwrap().path) {
        Ok(_) => (),
        Err(_) => return HttpResponse::BadRequest().json(Response{
            data:None,
            errors: Some(CustomError::DeletingFileError.error_response()),
        }),
    };
    match Info::delete_by_id(id.to_string()).exec(conn).await {
        Ok(_) =>return  HttpResponse::Ok().json(""),
        Err(_)=> return HttpResponse::BadRequest().json(Response{
            data:None,
            errors: Some(CustomError::DeletingFileError.error_response()),
        }),
    };
    
}

#[actix_web::main]


async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let conn : DatabaseConnection = sea_orm::Database::connect( dotenv::var("DATABASE_URL").unwrap()).await.expect("Error in conenction");
    
    
    match Migrator::up(&conn,None).await { 
        Ok(_) => println!("Migracije su okej"),
        Err(_) => println!("Migracije nisu okej")
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState{conn:conn.clone()}))
            .app_data(MultipartFormConfig::default().file_limit(25 * 1024 * 1024))
            .service(change_file)
            .service(delete_file)
            .service(save_file)
            .service(get_file)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}