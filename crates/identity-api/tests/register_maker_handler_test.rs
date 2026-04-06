use std::sync::Arc;

use async_trait::async_trait;
use axum::{body::Body, http::{Request, StatusCode}};
use http_body_util::BodyExt;
use identity_api::identity_router;
use identity_application::{ports::RegisterUserPort, use_cases::RegisterUserCommand};
use identity_domain::{
    errors::DomainError,
    events::UserRegistered,
    user::Role,
};
use shared_kernel::{ids::{IdentityId, new_id}, iso_date_time::IsoDateTime};
use tower::ServiceExt;

struct MockRegisterUserPort {
    result: Result<(), DomainError>,
}

#[async_trait]
impl RegisterUserPort for MockRegisterUserPort {
    async fn execute(
        &self,
        command: RegisterUserCommand,
    ) -> Result<(IdentityId, UserRegistered), DomainError> {
        self.result.as_ref().map_err(|e| match e {
            DomainError::Conflict(msg) => DomainError::Conflict(msg.clone()),
            DomainError::ValidationError(msg) => DomainError::ValidationError(msg.clone()),
            DomainError::NotFound => DomainError::NotFound,
            DomainError::Unauthorized => DomainError::Unauthorized,
            DomainError::Forbidden => DomainError::Forbidden,
        })?;

        let id = IdentityId(new_id());
        let event = UserRegistered {
            id,
            email: "maker@test.com".to_string(),
            role: Role::Maker,
            occurred_at: IsoDateTime::new("2026-04-04T00:00:00Z".to_string()).unwrap(),
        };
        Ok((id, event))
    }
}

fn post_json(uri: &str, body: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

#[tokio::test]
async fn inscription_maker_valide_retourne_201() {
    let uc = Arc::new(MockRegisterUserPort { result: Ok(()) });
    let app = identity_router(uc);

    let response = app
        .oneshot(post_json(
            "/api/v1/auth/register/maker",
            serde_json::json!({ "email": "maker@test.com", "password": "StrongPassword123", "name": "Mon Atelier" }),
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["email"], "maker@test.com");
    assert_eq!(json["role"], "Maker");
}

#[tokio::test]
async fn email_doublon_retourne_409_rfc7807() {
    let uc = Arc::new(MockRegisterUserPort {
        result: Err(DomainError::Conflict("email already exists".into())),
    });
    let app = identity_router(uc);

    let response = app
        .oneshot(post_json(
            "/api/v1/auth/register/maker",
            serde_json::json!({ "email": "maker@test.com", "password": "StrongPassword123", "name": "Mon Atelier" }),
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], 409);
    assert_eq!(json["title"], "Conflict");
}
