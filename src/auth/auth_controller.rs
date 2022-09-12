use actix_web::{get, post, web, HttpResponse};

use crate::{
    auth::{
        auth_service::{singin_user, singup_user},
        claims::Claims,
        dto::{LoginRequest, LoginResponse},
        jwt_service::create_jwt, auth_errors::AuthError,
    },
    context::AppState,
    user::dto::UserCreateDTO,
};

#[get("/singin")]
async fn singin(
    data: web::Data<AppState>,
    user: web::Json<LoginRequest>,
) -> Result<HttpResponse, AuthError> {
    let user_db = singin_user(data.clone(), user)
        .await
        .map_err(|e| return e)?;

    let token = create_jwt(
        Claims::new(
            user_db.username.to_string(),
            user_db.role.clone(),
            user_db.user_id.to_string(),
        ),
        data,
    )
    .map_err(|_| return AuthError::SignInError)?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        user: user_db,
        token,
    }))
}
#[post("/singup")]
async fn singup(
    data: web::Data<AppState>,
    user: web::Json<UserCreateDTO>,
) -> Result<HttpResponse, AuthError> {
    let new_user = singup_user(data, user).await.map_err(|r| return r)?;
    Ok(HttpResponse::Ok().json(new_user))
}
