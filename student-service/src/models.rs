use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Student {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub student_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewStudent {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub student_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegisteredEvent {
    pub event: String,
    pub data: UserData,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub created_at: String,
}