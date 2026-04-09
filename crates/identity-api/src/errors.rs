use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use identity_domain::errors::DomainError;
use serde_json::json;

pub struct ApiError {
    status: StatusCode,
    title: &'static str,
    detail: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = json!({
            "type": "about:blank",
            "title": self.title,
            "status": self.status.as_u16(),
            "detail": self.detail,
        });
        (self.status, Json(body)).into_response()
    }
}

impl From<DomainError> for ApiError {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::Conflict(msg) => ApiError {
                status: StatusCode::CONFLICT,
                title: "Conflict",
                detail: msg,
            },
            DomainError::ValidationError(msg) => ApiError {
                status: StatusCode::UNPROCESSABLE_ENTITY,
                title: "Unprocessable Entity",
                detail: msg,
            },
            DomainError::NotFound => ApiError {
                status: StatusCode::NOT_FOUND,
                title: "Not Found",
                detail: "resource not found".into(),
            },
            DomainError::Unauthorized => ApiError {
                status: StatusCode::UNAUTHORIZED,
                title: "Unauthorized",
                detail: "unauthorized".into(),
            },
            DomainError::Forbidden => ApiError {
                status: StatusCode::FORBIDDEN,
                title: "Forbidden",
                detail: "forbidden".into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conflict_mappe_vers_409() {
        let err = ApiError::from(DomainError::Conflict("email déjà utilisé".into()));
        assert_eq!(err.status, StatusCode::CONFLICT);
    }

    #[test]
    fn validation_error_mappe_vers_422() {
        let err = ApiError::from(DomainError::ValidationError(
            "mot de passe trop court".into(),
        ));
        assert_eq!(err.status, StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn not_found_mappe_vers_404() {
        let err = ApiError::from(DomainError::NotFound);
        assert_eq!(err.status, StatusCode::NOT_FOUND);
    }
}
