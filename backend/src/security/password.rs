use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

/// Hash a plaintext password with Argon2id.
pub fn hash_password(plain: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(plain.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Failed to hash password: {}", e))
}

/// Verify a plaintext password against a stored value.
///
/// Supports both Argon2 hashes and legacy plaintext rows (pre-hashing DB
/// data). Returns true on match; callers may upgrade legacy rows after a
/// successful legacy match.
pub fn verify_password(plain: &str, stored: &str) -> bool {
    if stored.starts_with("$argon2") {
        PasswordHash::new(stored)
            .map(|parsed| Argon2::default().verify_password(plain.as_bytes(), &parsed).is_ok())
            .unwrap_or(false)
    } else {
        stored == plain
    }
}

/// True when the stored value is an Argon2 hash rather than legacy plaintext.
pub fn is_hashed(stored: &str) -> bool {
    stored.starts_with("$argon2")
}