use actix_easy_multipart::extractor::MultipartForm;
use actix_web::{get, post, put, delete, HttpResponse, web};
use actix_web_grants::proc_macro::{has_any_role, has_permissions};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{context::AppState, errors::{ Errors}, file::{file_service::{getFile, createFile, deleteFile, changeFile}, file_manager::UploadData}, auth::{dto::UserClaims, jwt_service::decode_jwt}};



#[get("file/{id}")]
#[has_permissions("User","Admin")]
async fn get_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    credentials: BearerAuth,
) -> Result<HttpResponse, Errors> {

    let file_info = getFile(data,id,user_claims)
        .await.map_err(|e| return e)?;   
    Ok(HttpResponse::Ok().json(file_info))
}
#[post("file/{bucketId}")]
#[has_permissions("User","Admin")]
async fn create_file(
    data: web::Data<AppState>,
    bucket_id: web::Path<String>,
    payload: MultipartForm<UploadData>,
    credentials: BearerAuth,
) -> Result<HttpResponse, Errors> {
    let user_claims = decode_jwt(credentials.token())?;
    let file = createFile(data, bucket_id,payload,user_claims)
    .await.map_err(|e|  return e)?;
    Ok(HttpResponse::Ok().json(file))
}
#[put("file/{id}")]
#[has_permissions("User","Admin")]
async fn change_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    payload: MultipartForm<UploadData>,
    credentials: BearerAuth,
) -> Result<HttpResponse, Errors> {

    let file = changeFile(data,id,payload,user_claims)
    .await.map_err(|e| return e)?;


    Ok(HttpResponse::Ok().json(file))
}
#[delete("file/{id}")]
#[has_permissions("User","Admin")]
async fn delete_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    credentials: BearerAuth,
) -> Result<HttpResponse, Errors> {
   
    deleteFile(data, id,user_claims)
        .await.map_err(|e| return e)?;

    Ok(HttpResponse::Ok().finish())
}