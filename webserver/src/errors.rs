use core::fmt;

use actix_web::{error, http::StatusCode, HttpResponse};
use log::{error, info};
use serde::Serialize;
use sqlx::error::Error as SQLxError;

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    LoadError(String),
    NodeExist(String),
    NodeError(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl MyError {
    fn error_message(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                error!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            MyError::ActixError(msg) => {
                error!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            MyError::NotFound(msg) => {
                error!("Not found error occurred: {:?}", msg);
                msg.into()
            }
            MyError::NodeExist(msg) => {
                error!("节点已经存在: {:?}", msg);
                msg.into()
            }
            _ => "Something server error".into(),
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(_msg) | MyError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_message(),
        })
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

// use sqlx::error::Error as SQLxError;
impl From<SQLxError> for MyError {
    fn from(err: SQLxError) -> Self {
        MyError::DBError(err.to_string())
    }
}
