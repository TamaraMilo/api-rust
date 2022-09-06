

use entity::user::Role;
use super::dto::UserClaims;

pub async fn guard(user_claims: UserClaims) -> bool {
   
    match user_claims.role {
        Role::Admin => true,
        Role::User => true,
    } 
}

pub async fn guard_admin(user_claims: UserClaims) -> bool {
    match user_claims.role {
        Role::Admin => true,
        Role::User=> false
    }

}
