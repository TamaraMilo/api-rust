use chrono::{Duration, Utc};
use entity::user::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Claims {
    pub username: String,
    pub role: Role,
    pub user_id: String,
    exp: i64,
}

impl Claims {
    pub fn new(username: String, role: Role, user_id: String) -> Self {
        Self {
            username,
            role,
            user_id,
            exp: (Utc::now()+ Duration::hours(24)).timestamp(),
        }
    }
}

