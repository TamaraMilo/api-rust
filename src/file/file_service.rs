use super::dto::FileInfoDTO;
use super::file_manager::{ChangeArgs, FileManager, UploadData};
use super::file_repository::FileInfo;
use crate::auth::auth_service::user_verify;
use crate::auth::dto::UserClaims;
use crate::bucket::bucket_repository::Bucket;
use crate::repository::Reposiory;
use crate::{context::AppState, errors::Errors};
use actix_easy_multipart::extractor::MultipartForm;
use actix_web::web;
use entity::info::Model as FileModel;

pub async fn getFile(
    data: web::Data<AppState>,
    id: web::Path<String>,
    user_claims: UserClaims,
) -> Result<FileModel, Errors> {
    let file = FileInfo::new(data.conn.clone());
    let fileModule = file
        .read(id.to_string())
        .await
        .map_err(|_| return Errors::DatabaseError)?;
    if !user_verify(fileModule.user_id.to_string(), user_claims) {
        return Err(Errors::Unauthorized);
    }
    Ok(fileModule)
}

pub async fn createFile(
    data: web::Data<AppState>,
    bucket_id: web::Path<String>,
    mut payload: MultipartForm<UploadData>,
    user_claims: UserClaims,
) -> Result<FileModel, Errors> {
    let bucket = Bucket::new(data.conn.clone());
    let bucketModel = bucket
        .read(bucket_id.to_string())
        .await
        .map_err(|_| return Errors::BucketNotExisting)?;
    if !user_verify(bucketModel.user_id, user_claims.clone()) {
        return Err(Errors::Unauthorized);
    }
    let infoFileManager = FileManager::new(
        &mut payload.image,
        bucket_id.to_string(),
        &data.env_data.basic_storage,
    )
    .map_err(|_| return Errors::BucketNotExisting)?;

    let file = FileInfo::new(data.conn.clone());
    let fileModel = file
        .create(FileInfoDTO {
            id: infoFileManager.id.to_string(),
            extension: infoFileManager.extension.to_string(),
            path: infoFileManager.path.to_string(),
            user_id: user_claims.id.to_string(),
        })
        .await
        .map_err(|_| return Errors::DatabaseError)?;

    Ok(fileModel)
}

pub async fn changeFile(
    data: web::Data<AppState>,
    id: web::Path<String>,
    mut payload: MultipartForm<UploadData>,
    user_claims: UserClaims,
) -> Result<FileModel, Errors> {
    let file = FileInfo::new(data.conn.clone());
    let fileModel = file
        .read(id.to_string())
        .await
        .map_err(|_| return Errors::NoFileError)?;
    if !user_verify(fileModel.id.to_string(), user_claims.clone()) {
        return Err(Errors::Unauthorized);
    }

    let new_file_info = FileManager::change_data(ChangeArgs {
        data: &mut payload.image,
        file_info: &fileModel,
    })
    .map_err(|_| return Errors::ChangeFileError)?;

    let new_file = file
        .update(FileInfoDTO {
            id: new_file_info.id.to_string(),
            extension: new_file_info.extension,
            path: new_file_info.path,
            user_id: user_claims.id,
        })
        .await
        .map_err(|_| return Errors::ChangeFileError)?;
    Ok(new_file)
}

pub async fn deleteFile(
    data: web::Data<AppState>,
    id: web::Path<String>,
    user_claims: UserClaims,
) -> Result<(), Errors> {
    let file = FileInfo::new(data.conn.clone());
    let fileModel = file
        .read(id.to_string())
        .await
        .map_err(|_| return Errors::NoFileError)?;
    if !user_verify(fileModel.id.to_string(), user_claims.clone()) {
        return Err(Errors::Unauthorized);
    }

    FileManager::delete(fileModel.path).map_err(|_| return Errors::DeletingFileError)?;
    file.delete(id.to_string())
        .await
        .map_err(|_| return Errors::DeletingFileError)?;

    Ok(())
}
