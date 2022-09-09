use super::{bucket_manager::BucketManager, bucket_repository::Bucket, dto::BucketDTO};
use crate::{
    auth::{auth_service::user_verify, claims::Claims},
    context::AppState,
    errors::Errors,
    repository::Reposiory,
};
use actix_web::web;
use entity::bucket::Model as BucketModel;

pub async fn newBucket(
    data: web::Data<AppState>,
    name: web::Path<String>,
    user_claims: Claims,
) -> Result<BucketModel, Errors> {
    let bucket_repository = Bucket::new(data.conn.clone());
    let bucket  = bucket_repository.bucket_exist(name.to_string()).await.map_err(|_| return Errors::DatabaseError)?;
    if bucket.is_some()
    {
        return Err(Errors::BucketNameErrors);
    }
    let bucket_info = BucketManager::new(&data.env_data.basic_storage, name.to_string())
        .map_err(|_| return Errors::BucketCreateError)?;
   
    let bucketModel = bucket_repository
        .create(BucketDTO {
            bucket_id: bucket_info.id.to_string(),
            user_id: user_claims.user_id,
            name: name.to_string(),
        })
        .await
        .map_err(|_| return Errors::DatabaseError)?;

    Ok(bucketModel)
}

pub async fn deleteBucket(
    bucket_id: web::Path<String>,
    data: web::Data<AppState>,
    user_claims: Claims,
) -> Result<(), Errors> {
    let bucket = Bucket::new(data.conn.clone());
    let bucket_model = bucket
        .read(bucket_id.to_string())
        .await
        .map_err(|_| return Errors::DatabaseError)?;
    if !user_verify(bucket_model.user_id, user_claims) {
        return Err(Errors::Unauthorized);
    }
    BucketManager::delete(bucket_id.to_string(), &data.env_data.basic_storage)
        .map_err(|_| return Errors::BucketDeleteError)?;
    bucket
        .delete(bucket_id.to_string())
        .await
        .map_err(|_| return Errors::DatabaseError)?;
    Ok(())
}


