// BC Identity — handlers axum (thin HTTP adapters)
pub mod errors;
pub mod handlers;
pub mod router;

pub use router::identity_router;
