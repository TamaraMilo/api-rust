use actix_web::{delete, post, put, get, web, App, HttpResponse, HttpServer, HttpRequest};
mod errors;
use validator::{Validate};
mod file;
mod bucket;
mod structures;
mod responses;
use file::FileInfo;
use structures::{EnvData,LoginResponse,AppState,};
use sea_orm::{DatabaseConnection, ActiveModelTrait, Set, EntityTrait, QueryFilter, ColumnTrait};
use migration::{Migrator, MigratorTrait, Condition};
use actix_web::middleware::{Logger};
use entity::info::{ ActiveModel, Model};
use entity::info::Entity as Info;
use entity::user::{ActiveModel as ActiveModelUser};
use entity::user::Role;
use entity::user::Entity as User;
use entity::bucket::Entity as BucketEntity;
use entity::bucket::ActiveModel as ActiveModelBucket;
use actix_easy_multipart::extractor::{MultipartForm,MultipartFormConfig};
use dotenv::dotenv;
use bucket::Bucket;
use actix_jwt_auth_middleware::{Authority, AuthService};
use pwhash::bcrypt;
use crate::errors::{FileError, BucketError, LoggingError, DatabaseError};
use crate::responses::ResponseText;
use crate::structures::{Response, LogginData, UserData, Upload, validate, FileDetails, UserClaims};
#[macro_use]
extern crate lazy_static;




async fn verify_service_request(user_claims: UserClaims) ->bool{
    match user_claims.role {
        Role::Admin => true,
        Role::User=>true,
        _=>false,
    }
}
#[post("/logout")]
async fn logout(req: HttpRequest) -> HttpResponse{   
    let mut cookie = match req.cookie("auth-token") {
    Some(r)=>r,
    None=> return HttpResponse::BadRequest().json(Response{
        data: None,
        errors: Some(LoggingError::LoggedOut.error_response())
    })
    };
    cookie.make_removal();
    HttpResponse::Ok().cookie(cookie).json(Response{
        data:Some(ResponseText::LoggedOut.to_string()),
        errors:None
    })
}
#[get("/login")] 
async fn login( data: web::Data<AppState>, user: web::Json<LogginData>) -> HttpResponse {

    let conn=&data.conn;

    let user_db = match User::find().filter(Condition::any()
                        .add(entity::user::Column::Username.eq(user.identifier.to_string()))
                        .add(entity::user::Column::Email.eq(user.identifier.to_string()))).one(conn).await {
                            Ok(r)=>r,
                            Err(_) =>  return HttpResponse::BadRequest().json(Response{
                                data: None,
                                errors: Some(DatabaseError::DatabaseError.error_response()),
                            })
                        };
    let user_db = match user_db {
        Some(r) => r,
        None=> return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(LoggingError::IncorrectUserPassword.error_response())
        })
    };



    if !bcrypt::verify(user.password.to_string(), &user_db.password) {
        return HttpResponse::BadRequest().json(Response{
            data:None,
            errors: Some(LoggingError::IncorrectUserPassword.error_response())
        })
    }

    
    let mut cookie =  match data.auth.create_signed_cookie(UserClaims{
        id: user_db.user_id.to_string(),
        role: user_db.role.clone(),
    }){
        Ok(r)=>r,
        Err(_) =>return  HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(LoggingError::CreatingCookieError.error_response())
        })
    };

    cookie.set_secure(false);
    

    HttpResponse::Accepted().cookie(cookie).json(Response{
        data: Some(LoginResponse{
            user_id: user_db.user_id,
            username: user_db.username,
            email: user_db.email,
            role: user_db.role,
        }),
        errors: None,
    })
}
#[post("/sing-in")]
async fn sign_in(data: web::Data<AppState>, user: web::Json<UserData>) -> HttpResponse {
    let conn = &data.conn;
    match user.validate() {
        Ok(_)=>(),
        Err(_) => return HttpResponse::BadRequest().json(Response{
            data: None,
            errors:Some(LoggingError::PassAndUsernameError.error_response())
        })
    };
 

    let user_db = match User::find().filter(Condition::any()
                            .add(entity::user::Column::Username.eq(user.username.to_string()))
                            .add(entity::user::Column::Email.eq(user.email.to_string()))).one(conn).await {
                                Ok(r)=>r,
                                Err(_)=> return HttpResponse::BadRequest().json(Response{
                                    data:None,
                                    errors: Some(DatabaseError::DatabaseError.error_response())
                                })
                            };
    match user_db {
        Some(_)=> return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(LoggingError::UserExistError.error_response())
        }),
        None=>(),
    };
  
    let hash_password =match  bcrypt::hash(user.password.to_string()) {
        Ok(r)=>r,
        Err(_)=> return HttpResponse::BadRequest().json(Response{
            data:None,
            errors: Some(DatabaseError::DatabaseError.error_response()),
        })
    };

    let user_id = uuid::Uuid::new_v4().to_string();
    let account =ActiveModelUser{
        user_id: Set(user_id),
        username: Set(user.username.to_string()),
        password: Set(hash_password),
        email: Set(user.email.to_string()),
        role: Set(Role::default())

    };
    match account.insert(conn).await {
        Ok(user) => return HttpResponse::Ok().json(Response{
            data: Some(LoginResponse{
                user_id: user.user_id,
                username: user.username,
                email: user.email,
                role: user.role,
            }),
            errors: None,
        }),
        Err(_) => return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(LoggingError::NoUser.error_response())
        })
    }


}
#[put("/users/{user_id}/role")]
async fn user_admin(data: web::Data<AppState>, user_id: web::Path<String>,user_claims:UserClaims) -> HttpResponse {
    if user_claims.role != Role::Admin {
        return HttpResponse::Unauthorized().json(Response{
            data: None,
            errors: Some(()),
        })
    }
    
    let conn = &data.conn;

    let user = match User::find_by_id(user_id.to_string()).one(conn).await {
        Ok(r)=> r,
        Err(_)=> return HttpResponse::BadRequest().json(Response{
            data:None,
            errors:Some(DatabaseError::DatabaseError.error_response())
        })
    };
    let user = match user {
        Some(r)=> r,
        None=> return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(DatabaseError::DatabaseError.error_response())
        })
    };


    let mut user: ActiveModelUser = user.into();
    user.role = Set(Role::Admin);
    match user.update(conn).await {
        Ok(user)=>return HttpResponse::Ok().json(Response{
            data: Some(LoginResponse{
                user_id: user.user_id,
                username: user.username,
                email: user.email,
                role: user.role

            }),
            errors: None
        }),
        Err(_)=> return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(DatabaseError::DatabaseError.error_response())
        })
    };

}
#[get("/files/{id}")]
async fn get_file(data: web::Data<AppState>,id: web::Path<String>,user_claims:UserClaims) -> HttpResponse{
    
    let conn = &data.conn;
    let file_info = match Info::find_by_id(id.to_string()).one(conn).await {
        Ok(r)=> r,
        Err(_) => return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(FileError::NoFileError.error_response()),
        })
    };
   let file_info = match file_info {
        Some(r ) => r,
        None=> return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(FileError::NoFileError.error_response()) 
        })
        
    };
    
    if !validate(file_info.user_id.to_string(),user_claims.id,user_claims.role) {
        return HttpResponse::Unauthorized().json(Response{
            data: None,
            errors: Some(()),
        })
    }

    HttpResponse::Ok().json(Response{
        data: Some(file_info),
        errors: None
    })

}
#[post("/files/{bucketId}")]
async fn save_file_in_bucket(data: web::Data<AppState>,bucket_id: web::Path<String>, mut payload: MultipartForm<Upload>,user_claims: UserClaims) -> HttpResponse {
   
    let conn = &data.conn;

    let bucket = match BucketEntity::find_by_id(bucket_id.to_string()).one(conn).await{
        Ok(r)=> r,
        Err(_) => return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(DatabaseError::DatabaseError.error_response())
        })
    };

    let bucket = match bucket{
        Some(r)=> r,
        None => return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(BucketError::NoBucketError.error_response())
        })
    };
     
   
    if !validate(bucket.user_id.to_string(), user_claims.id.to_string(), user_claims.role) {
        return HttpResponse::Unauthorized().json(Response{
            data: None,
            errors: Some(()),
        })
    }

    
    let file_info = match FileInfo::new(&mut payload.image,bucket_id.to_string(),&data.env_data.basic_storage) {
        Ok(r)=>r,
        Err(_)=> return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(BucketError::BucketNotExisting.error_response())
        })
    };
    let file = ActiveModel {
        id: Set(file_info.id.to_string()),
        extension: Set(file_info.extension.to_string()),
        path: Set(file_info.path.to_string()),
        user_id: Set(user_claims.id.to_string())
    };
    match file.insert(conn).await {
        Ok(_)=>return  HttpResponse::Ok().json(
            Response{ 
                data: Some( Model {
                    id: file_info.id.to_string(),
                    extension: file_info.extension,
                    path: file_info.path,
                    user_id: user_claims.id
                }),
                errors: None
            }),
        Err(_)=> return HttpResponse::BadRequest().json(Response{
            data:None,
            errors: Some(DatabaseError::DatabaseError.error_response())
        })
    };
   
}
#[put("/files/{id}")]
async fn change_file(data: web::Data<AppState>,id: web::Path<String>,mut payload: MultipartForm<Upload>, user_claims: UserClaims) -> HttpResponse {
    
    let conn = &data.conn;

    let file = match Info::find_by_id(id.to_string()).one(conn).await{
        Ok(r)=>r,
        Err(_)=>return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(DatabaseError::DatabaseError.error_response())
        })
    };
    
  
    let file  = match file {
        Some(r) => r,
        None=> return HttpResponse::BadRequest().json(Response {
            data: None,
            errors: Some(FileError::NoFileError.error_response()),
        })
    };
   
    if !validate(file.user_id.to_string(), user_claims.id.to_string(), user_claims.role) {
        return HttpResponse::Unauthorized().json(Response{
            data: None,
            errors: Some(()),
        })
    }
    let new_file_info =match FileInfo::change_data(file::ChangeArgs{data:  &mut payload.image,file_info: &file}) {
        Ok(r)=>r,
        Err(_) => return HttpResponse::BadRequest().json(Response {
            data: None,
            errors: Some(FileError::ChangeFileError.error_response()),
        }),
    };

    let mut file: entity::info::ActiveModel = file.into();
    file.path = Set(new_file_info.path.clone());
    file.extension = Set(new_file_info.extension.to_string());
    match file.update(conn).await{
        Ok(_)=> return HttpResponse::Ok().json(Response {
            data: Some(FileDetails{
                id: id.to_string(),
                path: new_file_info.path,
                extension: new_file_info.extension .to_string(),
              
            }),
            errors: None,
        }),
        Err(_) => return HttpResponse::BadRequest().json(Response{
            data: None,
            errors:  Some(DatabaseError::DatabaseError.error_response())
        })
    };

    
    
}
#[delete("/files/{id}")]
async fn delete_file(data: web::Data<AppState>,id: web::Path<String>, user_claims: UserClaims) -> HttpResponse {
   
    let conn = &data.conn;

    let file = Info::find_by_id(id.to_string()).one(conn).await.unwrap();
    
    let file = match file {
        Some(r) => r,
        None=> return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(FileError::NoFileError.error_response())
        })
    };
    if !validate(file.user_id.to_string(), user_claims.id.to_string(), user_claims.role) {
        return HttpResponse::Unauthorized().json(Response{
            data: None,
            errors: Some(()),
        })
    }
    match FileInfo::delete( file.path) {
        Ok(_) => (),
        Err(_) => return HttpResponse::BadRequest().json(Response{
            data:None,
            errors: Some(FileError::DeletingFileError.error_response()),
        }),
    };
    match Info::delete_by_id(id.to_string()).exec(conn).await {
        Ok(_) =>return  HttpResponse::Ok().json(Response{
            data: Some(()),
            errors: None,
        }),
        Err(_)=> return HttpResponse::BadRequest().json(Response{
            data:None,
            errors: Some(FileError::DeletingFileError.error_response()),
        }),
    };
    
}
#[post("/bucket")]
async fn new_bucket(data: web::Data<AppState>,user_claims:UserClaims) -> HttpResponse {
    
    let conn = &data.conn;

    let bucket_id = match Bucket::new(&data.env_data.basic_storage) {
        Ok(r) => r,
        Err(_)=> return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(BucketError::BucketCreateError.error_response()),
        })
    };
    let bucket = ActiveModelBucket{
        bucket_id: Set(bucket_id.id.to_string()),
        user_id: Set(user_claims.id.to_string())
    };
    match bucket.insert(conn).await {
        Ok(r)=> return HttpResponse::Ok().json(
            Response{
                data: Some(r),
                errors:None
            }
        ),
        Err(_)=> return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(DatabaseError::DatabaseError.error_response())
        })
        
    };
}
#[delete("/bucket/{bucket_id}")]
async fn delete_bucket(bucket_id: web::Path<String>,data: web::Data<AppState>,user_claims:UserClaims) -> HttpResponse {
    
    let conn = &data.conn;
    let  bucket = match BucketEntity::find_by_id(bucket_id.to_string()).one(conn).await{
        Ok(r)=> r,
        Err(_) => return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(DatabaseError::DatabaseError.error_response())
        })
    };

    let bucket = match bucket{
        Some(r)=> r,
        None => return HttpResponse::BadRequest().json(Response{
            data: None,
            errors: Some(BucketError::NoBucketError.error_response())
        })
    };
     
    if !validate(bucket.user_id.to_string(), user_claims.id.to_string(), user_claims.role) {
        return HttpResponse::Unauthorized().json(Response{
            data: None,
            errors: Some(()),
        })
    }
    match Bucket::delete(bucket_id.to_string(),&data.env_data.basic_storage) {
        Ok(())=> return HttpResponse::Ok().json(Response{
            data: Some(()),
            errors: None,
        }),
        Err(_)=> return HttpResponse::BadRequest().json(Response{
            data: None,
            errors:Some(BucketError::BucketDeleteError.error_response()),
        })
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
