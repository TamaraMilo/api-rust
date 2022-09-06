use actix_easy_multipart::extractor::MultipartForm;
use actix_web::{get, post, put, delete, HttpResponse, web};
use actix_web_grants::proc_macro::has_any_role;

use crate::{context::AppState, errors::{ Errors}, file::{file_service::{getFile, createFile, deleteFile, changeFile}, file_manager::UploadData}, auth::dto::UserClaims};



#[get("{id}")]
#[has_any_role("User","Admin")]
async fn get_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    user_claims: UserClaims,
) -> Result<HttpResponse, Errors> {

    let file_info = getFile(data,id,user_claims)
        .await.map_err(|e| return e)?;   
    Ok(HttpResponse::Ok().json(file_info))
}
#[post("{bucketId}")]
#[has_any_role("User","Admin")]
async fn create_file(
    data: web::Data<AppState>,
    bucket_id: web::Path<String>,
    payload: MultipartForm<UploadData>,
    user_claims: UserClaims,
) -> Result<HttpResponse, Errors> {
    
    let file = createFile(data, bucket_id,payload,user_claims)
    .await.map_err(|e|  return e)?;
    Ok(HttpResponse::Ok().json(file))
}
#[put("{id}")]
#[has_any_role("User","Admin")]
async fn change_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    payload: MultipartForm<UploadData>,
    user_claims: UserClaims,
) -> Result<HttpResponse, Errors> {

    let file = changeFile(data,id,payload,user_claims)
    .await.map_err(|e| return e)?;


    Ok(HttpResponse::Ok().json(file))
}
#[delete("{id}")]
#[has_any_role("User","Admin")]
async fn delete_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    user_claims: UserClaims,
) -> Result<HttpResponse, Errors> {
   
    deleteFile(data, id,user_claims)
        .await.map_err(|e| return e)?;

    Ok(HttpResponse::Ok().finish())
}