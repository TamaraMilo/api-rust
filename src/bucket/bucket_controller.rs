use crate::{
    auth::dto::UserClaims,
    bucket::bucket_service::{deleteBucket, newBucket},
    context::AppState,
    errors::Errors,
};
use actix_web::{delete, post, web, HttpResponse};
use actix_web_grants::proc_macro::{ has_any_role};


#[post("/")]
#[has_any_role("User","Admin")]
async fn new_bucket(
    data: web::Data<AppState>,
    user_claims: UserClaims,
) -> Result<HttpResponse, Errors> {
    let bucket = newBucket(data, user_claims)
        .await
        .map_err(|_| return Errors::BucketCreateError)?;
    Ok(HttpResponse::Ok().json(bucket))
}

#[delete("/{bucket_id}")]
#[has_any_role("User","Admin")]
async fn delete_bucket(
    bucket_id: web::Path<String>,
    data: web::Data<AppState>,
    user_claims: UserClaims,
) -> Result<HttpResponse, Errors> {
    deleteBucket(bucket_id, data, user_claims)
        .await
        .map_err(|e| e)?;
    Ok(HttpResponse::Ok().finish())
}
