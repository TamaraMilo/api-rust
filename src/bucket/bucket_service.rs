use super::{bucket_manager::BucketManager, bucket_repository::Bucket, dto::BucketDTO, bucket_errors::BucketError};
use crate::{
    auth::{ claims::Claims, auth_service::user_authentication},
    context::AppState,
    repository::Reposiory,
};
use actix_web::web;
use entity::bucket::Model as BucketModel;

pub async fn create_bucket(
    data: web::Data<AppState>,
    name: web::Path<String>,
    user_claims: Claims,
) -> Result<BucketModel, BucketError> {
    let bucket_repository = Bucket::new(data.conn.clone());
    let bucket  = bucket_repository.bucket_exist(name.to_string()).await.map_err(|_| return BucketError::BucketCreateError)?;
    if bucket.is_some()
    {
        return Err(BucketError::BucketNameError);
    }
    let bucket_info = BucketManager::new(&data.env_data.basic_storage, name.to_string())
        .map_err(|_| return BucketError::BucketCreateError)?;
   
    let bucket_model = bucket_repository
        .create(BucketDTO {
            bucket_id: bucket_info.id.to_string(),
            user_id: user_claims.user_id,
            name: name.to_string(),
        })
        .await
        .map_err(|_| return BucketError::BucketCreateError)?;

    Ok(bucket_model)
}

pub async fn remove_bucket(
    name: web::Path<String>,
    data: web::Data<AppState>,
    user_claims: Claims,
) -> Result<(), BucketError> {
    let bucket = Bucket::new(data.conn.clone());
    let bucket_model = bucket
        .read(name.to_string())
        .await
        .map_err(|_| return BucketError::BucketDeleteError)?;
    if !user_authentication(bucket_model.user_id, user_claims) {
        return Err(BucketError::Unauthorized);
    }
    BucketManager::delete(name.to_string(), &data.env_data.basic_storage)
        .map_err(|_| return BucketError::BucketDeleteError)?;
    bucket
        .delete(bucket_model.bucket_id.to_string())
        .await
        .map_err(|_| return BucketError::BucketDeleteError)?;
    Ok(())
}


