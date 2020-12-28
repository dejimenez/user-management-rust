use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "JWKSFetchError")]
    JWKSFetchError,

     #[display(fmt = "Forbidden")]
    Forbidden,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::JWKSFetchError => {
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            }
            ServiceError::Forbidden => {
                HttpResponse::Forbidden().json("User or password incorrect")
            }
        }
    }
}
