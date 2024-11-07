use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use sqlx::Error as SQLxError;


#[derive(Debug,Serialize)]
pub enum MyError {
    DBError(String),//DbError(sqlx::Error),
    NotFound(String),
    ActixError(String),
}
#[derive(Debug,Serialize)]
pub struct MyErrorResponse{
    error_message: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(e) => format!("DBError: {:?}", e),
            MyError::NotFound(e) => format!("NotFound: {:?}", e),
            MyError::ActixError(e) => format!("ActixError: {:?}", e),
        }
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::DBError(e) => write!(f, "DBError: {}", e),
            MyError::NotFound(e) => write!(f, "NotFound: {}", e),
            MyError::ActixError(e) => write!(f, "ActixError: {}", e),
        }
    }
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(_)| MyError::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for MyError {
    fn from(value: SQLxError) -> Self {
        MyError::DBError(value.to_string())
    }
}