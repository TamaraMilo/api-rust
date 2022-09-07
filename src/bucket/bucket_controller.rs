use crate::{
    auth::jwt_service::decode_jwt,
    bucket::bucket_service::{deleteBucket, newBucket},
    context::AppState,
    errors::Errors,
};
use actix_web::{delete, post, web, HttpResponse};
use actix_web_grants::proc_macro::has_any_role;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use entity::user::Role::{self, Admin, User};

#[post("bucket/")]
#[has_any_role("Admin", "User", type = "Role")]
async fn new_bucket(
    data: web::Data<AppState>,
    credentials: BearerAuth,
) -> Result<HttpResponse, Errors> {
    let user_claims = decode_jwt(credentials.token()).map_err(|_| return Errors::Unauthorized)?;
    let bucket = newBucket(data, user_claims)
        .await
        .map_err(|_| return Errors::BucketCreateError)?;
    Ok(HttpResponse::Ok().json(bucket))
}

#[delete("bucket/{bucket_id}")]
#[has_any_role("Admin", "User", type = "Role")]
async fn delete_bucket(
    bucket_id: web::Path<String>,
    data: web::Data<AppState>,
    credentials: BearerAuth,
) -> Result<HttpResponse, Errors> {
    let user_claims = decode_jwt(credentials.token()).map_err(|_| return Errors::Unauthorized)?;
    deleteBucket(bucket_id, data, user_claims)
        .await
        .map_err(|e| e)?;
    Ok(HttpResponse::Ok().finish())
}
