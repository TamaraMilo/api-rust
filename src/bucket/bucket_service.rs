use super::{bucket_manager::BucketManager, bucket_repository::Bucket, dto::BucketDTO};
use crate::{
    auth::{auth_service::user_verify, dto::UserClaims},
    context::AppState,
    errors::Errors,
    repository::Reposiory,
};
use actix_web::web;
use entity::bucket::Model as BucketModel;

pub async fn newBucket(
    data: web::Data<AppState>,
    user_claims: UserClaims,
) -> Result<BucketModel, Errors> {
    let bucketInfo = BucketManager::new(&data.env_data.basic_storage)
        .map_err(|_| return Errors::BucketCreateError)?;
    let bucket = Bucket::new(data.conn.clone());
    let bucketModel = bucket
        .create(BucketDTO {
            bucket_id: bucketInfo.id.to_string(),
            user_id: user_claims.id,
        })
        .await
        .map_err(|_| return Errors::DatabaseError)?;

    Ok(bucketModel)
}

pub async fn deleteBucket(
    bucket_id: web::Path<String>,
    data: web::Data<AppState>,
    user_claims: UserClaims,
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