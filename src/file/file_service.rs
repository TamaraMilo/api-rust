use super::dto::{ChangeFile, FileInfoDTO};
use super::file_errors::FileError;
use super::file_manager::{ChangeArgs, FileManager, UploadData};
use super::file_repository::{ FileInfo};
use crate::auth::auth_service::{ user_authentication};
use crate::auth::claims::Claims;
use crate::bucket::bucket_repository::Bucket;
use crate::repository::Reposiory;
use crate::{context::AppState};
use actix_easy_multipart::extractor::MultipartForm;
use actix_web::{web};
use entity::info::Model as FileModel;


pub async fn read_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    user_claims: Claims,
) -> Result<FileModel, FileError> {
    let file = FileInfo::new(data.conn.clone());
    let file_module = file
        .read(id.to_string())
        .await
        .map_err(|_| return FileError::ReadingFileError)?;
    if !user_authentication(file_module.user_id.to_string(), user_claims) {
        return Err(FileError::Unauthorized);
    }
    Ok(file_module)
}

pub async fn save_file(
    data: web::Data<AppState>,
    mut payload: MultipartForm<UploadData>,
    user_claims: Claims,
) -> Result<FileModel, FileError> {
    let bucket = Bucket::new(data.conn.clone());
    let bucket_model = bucket
        .read(payload.bucket.to_string())
        .await
        .map_err(|_| return FileError::CreateFileError)?;
    if !user_authentication(bucket_model.user_id, user_claims.clone()) {
        return Err(FileError::Unauthorized);
    }
    let info_file_manager = FileManager::new(
        &mut payload.image,
        bucket_model.name,
        &data.env_data.basic_storage,
    )
    .map_err(|_| return FileError::NoBucketError)?;

    let file = FileInfo::new(data.conn.clone());
    let file_model = file
        .create(FileInfoDTO {
            id: info_file_manager.id.to_string(),
            extension: info_file_manager.extension.to_string(),
            path: info_file_manager.path.to_string(),
            user_id: user_claims.user_id.to_string(),
            bucket_id: payload.bucket.to_string(),
        })
        .await
        .map_err(|_| return FileError::CreateFileError)?;

    Ok(file_model)
}

pub async fn update_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    mut payload: MultipartForm<ChangeFile>,
    user_claims: Claims,
) -> Result<FileModel, FileError> {
    let file = FileInfo::new(data.conn.clone());
    let file_model = file
        .read(id.to_string())
        .await
        .map_err(|_| return FileError::NoFileError)?;
    if !user_authentication(file_model.id.to_string(), user_claims.clone()) {
        return Err(FileError::Unauthorized);
    }

    let new_file_info = FileManager::change_data(ChangeArgs {
        data: &mut payload.image,
        file_info: &file_model,
    })
    .map_err(|_| return FileError::ChangeFileError)?;

    let new_file = file
        .update(FileInfoDTO {
            id: new_file_info.id.to_string(),
            extension: new_file_info.extension,
            path: new_file_info.path,
            user_id: user_claims.user_id.to_string(),
            bucket_id: file_model.bucket_id.to_string(),
        })
        .await
        .map_err(|_| return FileError::ChangeFileError)?;
    Ok(new_file)
}

pub async fn remove_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    user_claims: Claims,
) -> Result<(), FileError> {
    let file = FileInfo::new(data.conn.clone());
    let file_model = file
        .read(id.to_string())
        .await
        .map_err(|_| return FileError::NoFileError)?;
    if !user_authentication(file_model.id.to_string(), user_claims) {
        return Err(FileError::Unauthorized);
    }

    FileManager::delete(file_model.path).map_err(|_| return FileError::DeletingFileError)?;
    file.delete(id.to_string())
        .await
        .map_err(|_| return FileError::DeletingFileError)?;

    Ok(())
}

pub async fn read_files_from_bucket_in_page(
    data: web::Data<AppState>,
    name: web::Path<String>,
    page_number: web::Path<usize>,
    user_claims: Claims,
) -> Result<Vec<FileModel>, FileError> {
    let bucket_repository = Bucket::new(data.conn.clone());
    let bucket = bucket_repository
        .read(name.to_string())
        .await
        .map_err(|_| return FileError::NoBucketError)?;

    if !user_authentication(bucket.user_id, user_claims) {
        return Err(FileError::Unauthorized);
    }

    let file_repository = FileInfo::new(data.conn.clone());
    file_repository
        .read_page(bucket.bucket_id.to_string(), page_number.to_be(), data.env_data.page_size)
        .await
        .map_err(|_| return FileError::ReadingFileError)
}
