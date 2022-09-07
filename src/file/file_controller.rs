use actix_easy_multipart::extractor::MultipartForm;
use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web_grants::proc_macro::has_any_role;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use entity::user::Role::{self, Admin, User};

use crate::{
    auth::jwt_service::decode_jwt,
    context::AppState,
    errors::Errors,
    file::{
        file_manager::UploadData,
        file_service::{changeFile, createFile, deleteFile, getFile}, dto::ChangeFile,
    },
};

#[get("file/{id}")]
#[has_any_role("Admin", "User", type = "Role")]
async fn get_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    credentials: BearerAuth,
) -> Result<HttpResponse, Errors> {
    let user_claims = decode_jwt(credentials.token()).map_err(|_| return Errors::Unauthorized)?;
    let file_info = getFile(data, id, user_claims).await.map_err(|e| return e)?;
    Ok(HttpResponse::Ok().json(file_info))
}
#[post("file/{bucketId}")]
#[has_any_role("Admin", "User", type = "Role")]
async fn create_file(
    data: web::Data<AppState>,
    bucket_id: web::Path<String>,
    payload: MultipartForm<UploadData>,
    credentials: BearerAuth,
) -> Result<HttpResponse, Errors> {
    let user_claims = decode_jwt(credentials.token()).map_err(|_| return Errors::Unauthorized)?;
    let file = createFile(data, payload, user_claims)
        .await
        .map_err(|e| return e)?;
    Ok(HttpResponse::Ok().json(file))
}
#[put("file/{id}")]
#[has_any_role("Admin", "User", type = "Role")]
async fn change_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    payload: MultipartForm<ChangeFile>,
    credentials: BearerAuth,
) -> Result<HttpResponse, Errors> {
    let user_claims = decode_jwt(credentials.token()).map_err(|_| return Errors::Unauthorized)?;
    let file = changeFile(data, id, payload, user_claims)
        .await
        .map_err(|e| return e)?;

    Ok(HttpResponse::Ok().json(file))
}
#[delete("file/{id}")]
#[has_any_role("Admin", "User", type = "Role")]
async fn delete_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    credentials: BearerAuth,
) -> Result<HttpResponse, Errors> {
    let user_claims = decode_jwt(credentials.token()).map_err(|_| return Errors::Unauthorized)?;
    deleteFile(data, id, user_claims)
        .await
        .map_err(|e| return e)?;

    Ok(HttpResponse::Ok().finish())
}
