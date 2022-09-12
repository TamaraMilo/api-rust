use crate::{
    auth::jwt_service::decode_jwt,
    bucket::{bucket_service::{remove_bucket, create_bucket}, bucket_errors::BucketError},
    context::AppState,
};
use actix_web::{delete, post, web, HttpResponse};
use actix_web_grants::proc_macro::has_any_role;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use entity::user::Role::{self, Admin, User};

#[post("{name}")]
#[has_any_role("Admin", "User", type = "Role")]
async fn new_bucket(
    data: web::Data<AppState>,
    name: web::Path<String>,
    credentials: BearerAuth,
) -> Result<HttpResponse, BucketError> {
    let user_claims = decode_jwt(credentials.token()).map_err(|_| return BucketError::Unauthorized)?;
    let bucket = create_bucket(data, name, user_claims)
        .await
        .map_err(|e| return e)?;
    Ok(HttpResponse::Ok().json(bucket))
}

#[delete("{name}")]
#[has_any_role("Admin", "User", type = "Role")]
async fn delete_bucket(
    name: web::Path<String>,
    data: web::Data<AppState>,
    credentials: BearerAuth,
) -> Result<HttpResponse, BucketError> {
    let user_claims = decode_jwt(credentials.token()).map_err(|_| return BucketError::Unauthorized)?;
    remove_bucket(name, data, user_claims).await.map_err(|e| e)?;
    Ok(HttpResponse::Ok().finish())
}

