use actix_web::{ HttpResponse, web, put};
use actix_web_grants::proc_macro::{has_any_role, has_permissions};
use crate::{errors::Errors, context::AppState, user::{user_repository::{ update_user}, }, auth::dto::{UserData, UserClaims}};



#[put("/{user_id}/role")]
#[has_permissions("Admin")]
async fn user_admin(
    data: web::Data<AppState>,
    user_id: web::Path<String>
) -> Result<HttpResponse, Errors> {
    let conn = &data.conn;

    let user = update_user(conn, user_id.to_string())
        .await
        .map_err(|_| return Errors::DatabaseError)?;

    Ok(HttpResponse::Ok().json(UserData {
        user_id: user.user_id,
        username: user.username,
        email: user.email,
        role: user.role,
    }))
}