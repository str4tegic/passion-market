use identity_application::ports::PasswordHasher;
use identity_domain::password_hash::PasswordHash;
use identity_infra::argon2_hasher::Argon2PasswordHasher;

#[test]
fn valid_password_starts_with_argon2() {
    let hasher = Argon2PasswordHasher;
    let result = hasher.hash_password("motdepasse123").unwrap();
    assert!(result.0.starts_with("$argon2"));
}

#[test]
fn too_short_password_returns_error() {
    let hasher = Argon2PasswordHasher;
    let result = hasher.hash_password("court");
    assert!(result.is_err());
}

#[test]
fn hash_retourne_un_password_hash() {
    let hasher = Argon2PasswordHasher;
    let result: Result<PasswordHash, _> = hasher.hash_password("motdepasse123");
    assert!(result.is_ok());
}