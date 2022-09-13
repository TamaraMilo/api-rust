use actix_easy_multipart::extractor::MultipartForm;
use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web_grants::proc_macro::has_any_role;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use entity::user::Role::{self, Admin, User};

use crate::{
    auth::jwt_service::decode_jwt,
    context::AppState,
    file::{
        file_manager::UploadData,
        file_service::{update_file, save_file, remove_file, read_file, read_files_from_bucket_in_page, read_file_url}, dto::ChangeFile, file_errors::FileError,
    },
};

#[get("/{id}")]
#[has_any_role("Admin", "User", type = "Role")]
async fn get_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    credentials: BearerAuth,
) -> Result<HttpResponse, FileError> {
    let user_claims = decode_jwt(credentials.token()).map_err(|_| return FileError::Unauthorized)?;
    let file_info = read_file(data, id, user_claims).await.map_err(|e| return e)?;
    Ok(HttpResponse::Ok().json(file_info))
}

#[get("{name}/page/{page_number}")]
#[has_any_role("Admin", "User", type="Role")]
async fn get_files_page(data: web::Data<AppState>, name: web::Path<String>, page_number: web::Path<usize>, credentials: BearerAuth, ) -> Result<HttpResponse, FileError> {

    let user_claims = decode_jwt(credentials.token()).map_err(|_| return FileError::Unauthorized)?;
    let files = read_files_from_bucket_in_page(data, name,page_number,user_claims).await.map_err(|e| return e)?;
    Ok(HttpResponse::Ok().json(files))
    
}


#[post("")]
#[has_any_role("Admin", "User", type = "Role")]
async fn create_file(
    data: web::Data<AppState>,
    payload: MultipartForm<UploadData>,
    credentials: BearerAuth,
) -> Result<HttpResponse, FileError> {
    let user_claims = decode_jwt(credentials.token()).map_err(|_| return FileError::Unauthorized)?;
    let file = save_file(data, payload, user_claims)
        .await
        .map_err(|e| return e)?;
    Ok(HttpResponse::Ok().json(file))
}
#[put("{id}")]
#[has_any_role("Admin", "User", type = "Role")]
async fn change_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    payload: MultipartForm<ChangeFile>,
    credentials: BearerAuth,
) -> Result<HttpResponse, FileError> {
    let user_claims = decode_jwt(credentials.token()).map_err(|_| return FileError::Unauthorized)?;
    let file = update_file(data, id, payload, user_claims)
        .await
        .map_err(|e| return e)?;

    Ok(HttpResponse::Ok().json(file))
}
#[delete("{id}")]
#[has_any_role("Admin", "User", type = "Role")]
async fn delete_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    credentials: BearerAuth,
) -> Result<HttpResponse, FileError> {
    let user_claims = decode_jwt(credentials.token()).map_err(|_| return FileError::Unauthorized)?;
    remove_file(data, id, user_claims)
        .await
        .map_err(|e| return e)?;

    Ok(HttpResponse::Ok().finish())
}
#[get("/{bucket_name}/{file}")]
async fn show_file_url(data: web::Data<AppState>, bucket_name: web::Path<(String,String)>) -> Result<HttpResponse,FileError>
{
    read_file_url(data,bucket_name.0.to_string(),bucket_name.1.to_string())
}

