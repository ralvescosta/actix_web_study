use crate::view_models::http_error::HttpError;
use actix_web::{error, error::JsonPayloadError, Error, HttpRequest, HttpResponse};

pub fn error_json(err: JsonPayloadError, req: &HttpRequest) -> Error {
    error::InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(HttpError {
            status_code: 400,
            message: String::from("Wrong Body format"),
            details: String::from("Some fields dot match with the documentation"),
        }),
    )
    .into()
}
