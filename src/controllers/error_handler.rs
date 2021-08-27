use crate::view_models::http_error::HttpError;
use actix_web::{error, error::JsonPayloadError, Error, HttpRequest, HttpResponse};

pub fn error_json(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    error::InternalError::from_response(
        format!("JSON error: {:?}", err),
        HttpResponse::BadRequest().json(HttpError {
            status_code: 400,
            message: String::from("Wrong body format"),
            details: String::from(format!("{}", err)),
        }),
    )
    .into()
}
