use chrono::{Duration, Utc};
use entity::user::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub role: Role,
    exp: i64,
}

impl Claims {
    pub fn new(username: String, role: Role) -> Self {
        Self {
            username,
            role,
            exp: (Utc::now()+ Duration::hours(24)).timestamp(),
        }
    }
}

