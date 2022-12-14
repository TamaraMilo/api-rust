use super::{claims::Claims, auth_errors::AuthError};
use crate::{context::AppState};
use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, web, Error};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

pub fn create_jwt(claims: Claims, data: web::Data<AppState>) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(data.env_data.secret_key.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}

pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let key = dotenv::var("SECRET").map_err(|_| return AuthError::SignInError)?;
    let decoding_key = DecodingKey::from_secret(key.as_bytes());
    jsonwebtoken::decode(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| return ErrorUnauthorized(e.to_string()))
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let result = decode_jwt(credentials.token());
    match result {
        Ok(claims) => {
            req.attach(vec![claims.role]);
            Ok(req)
        }
        Err(e) => Err((e, req)),
    }
}
