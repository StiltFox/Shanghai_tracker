use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::sqlx;

#[derive(Serialize, Deserialize,Responder)]
#[serde(crate = "rocket::serde")]
#[response(status = 500, content_type = "json")]
pub struct DatabaseError {
    message: String,
}

impl From<sqlx::Error> for DatabaseError {
    fn from(value: sqlx::Error) -> Self {
        DatabaseError {
            message: value.to_string()
        }
    }
}

impl From<uuid::Error> for DatabaseError {
    fn from(value: uuid::Error) -> Self {
        DatabaseError {
            message: value.to_string()
        }
    }
}