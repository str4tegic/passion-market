use std::sync::Arc;

use axum::{routing::post, Router};
use identity_application::ports::RegisterUserPort;

use crate::handlers::register_maker::register_maker;

pub fn identity_router(uc: Arc<dyn RegisterUserPort>) -> Router {
    Router::new()
        .route("/api/v1/auth/register/maker", post(register_maker))
        .with_state(uc)
}
