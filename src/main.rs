
use actix_web::{delete, post, put, get, web, App, HttpResponse, HttpServer};
mod errors;
mod file;
use actix_multipart::Multipart;
use file::FileInfo;
use sea_orm::{DatabaseConnection, ActiveModelTrait, Set, EntityTrait};
use migration::{Migrator, MigratorTrait};
use actix_web::middleware::Logger;
use entity::info::{ ActiveModel, Model, self};
use entity::info::Entity as Info;
use errors::CustomError;


const DATABASE_URL:&str = "postgres://postgres:Nignite@192.168.1.88";

#[derive(Debug,Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[get("/files/{id}")]
async fn get_extension(data: web::Data<AppState>,id: web::Path<String>) -> HttpResponse{

    let conn = &data.conn;
    let _file =match Info::find_by_id(id.to_string()).one(conn).await.unwrap() {
        Some(r)=> return  HttpResponse::Ok().content_type("json").json(r),
        None => return HttpResponse::BadRequest().json(CustomError::NoFileError.to_string())
    };
   
}

#[post("/files")]
async fn save_file(data: web::Data<AppState>,payload: Multipart) -> HttpResponse {

    let conn = &data.conn;

    let basic_info = FileInfo::parse_file_data(payload).await;
    let file_info = match FileInfo::new_file(basic_info.bytes, basic_info.extension.clone()) {
        Ok(r) => r,
        Err(e) => return HttpResponse::BadRequest().json(e.error_response())
    };
    let file = ActiveModel {
        id: Set(file_info.id.to_string()),
        extension: Set(basic_info.extension),
        path: Set(file_info.path.to_string())
    };
    let _file: Model = file.insert(conn).await.expect("Error");
    HttpResponse::Ok().json(file_info.id)
    
}


#[put("/files/{id}")]
async fn change_file(data: web::Data<AppState>,id: web::Path<String>, payload: Multipart) -> HttpResponse {

    let conn = &data.conn;

    let file = Info::find_by_id(id.to_string()).one(conn).await.unwrap();
    
    if file == None {
        return HttpResponse::BadRequest().content_type("json").json(CustomError::NoFileError.error_response())
    }
  
    let data = FileInfo::parse_file_data(payload).await;
    let new_path =match FileInfo::change_data(file::ChangeArgs{data: data.bytes, extension: data.extension.clone(), path: file.as_ref().unwrap().path.clone()}) {
        Ok(r)=>r,
        Err(e) => return HttpResponse::BadRequest().content_type("json").json(e.to_string()),
    };
    let mut file: entity::info::ActiveModel = file.unwrap().into();
    file.path = Set(new_path.clone());
    file.extension = Set(data.extension);
    let _file : info::Model = file.update(conn).await.unwrap();
    HttpResponse::Ok().content_type("json").json(new_path)
    
}


#[delete("/files/{id}")]
async fn delete_file(data: web::Data<AppState>,id: web::Path<String>) -> HttpResponse {

    let conn = &data.conn;

    let file = Info::find_by_id(id.to_string()).one(conn).await.unwrap();
    
    if file == None {
        return HttpResponse::BadRequest().content_type("json").json(CustomError::NoFileError.to_string())
    }

    let result=  match FileInfo::delete( file.unwrap().path) {
            Ok(result) => result,
            Err(e) => return HttpResponse::BadRequest().json(e.error_response()),
        };
       match Info::delete_by_id(id.to_string()).exec(conn).await {
            Ok(_r) =>return  HttpResponse::Ok().content_type("json").json(result),
            Err(e)=> return HttpResponse::BadRequest().json(e.to_string()),
        };
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let conn : DatabaseConnection = sea_orm::Database::connect(DATABASE_URL).await.expect("Error in conenction");
    
    match Migrator::up(&conn,None).await { 
        Ok(_) => println!("Migracije su okej"),
        Err(_) => println!("Migracije nisu okej")
    };


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState{conn:conn.clone()}))
            .service(change_file)
            .service(delete_file)
            .service(save_file)
            .service(get_extension)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
