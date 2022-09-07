use actix_web::{get, post, web, HttpRequest, HttpResponse};

use crate::{
    auth::{
        auth_service::{singin_user, singup_user},
        claims::Claims,
        dto::{LoginRequest, LoginResponse},
        jwt_service::create_jwt,
    },
    context::AppState,
    errors::Errors,
    responses::ResponseText,
    user::dto::UserCreateDTO,
};

#[post("/singout")]
async fn singout(req: HttpRequest) -> Result<HttpResponse, Errors> {
    let mut cookie = match req.cookie("auth-token") {
        Some(r) => r,
        None => return Err(Errors::LoggedOut),
    };
    cookie.make_removal();
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(ResponseText::LoggedOut.to_string()))
}

#[get("/singin")]
async fn singin(
    data: web::Data<AppState>,
    user: web::Json<LoginRequest>,
) -> Result<HttpResponse, Errors> {
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
    .map_err(|_| return Errors::SingInError)?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        user: user_db,
        token,
    }))
}
#[post("/singup")]
async fn singup(
    data: web::Data<AppState>,
    user: web::Json<UserCreateDTO>,
) -> Result<HttpResponse, Errors> {
    let new_user = singup_user(data, user).await.map_err(|r| return r)?;
    Ok(HttpResponse::Ok().json(new_user))
}
