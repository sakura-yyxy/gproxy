//! Wrap / unwrap upstream realtime `call_id`s with a keyed signature so
//! downstream clients cannot use another user's call identifier.
//!
//! Format: `rtc_{user_id}_{sig16hex}_{real_tail}` where `real_tail` is the
//! upstream `call_id` with its leading `rtc_` stripped, and `sig16hex` is the
//! first 16 hex characters (8 bytes) of
//! `sha256(secret || "|" || user_id || "|" || real)`. Decoding requires the
//! exact authenticated `user_id`, so leaking a wrapped id to another user
//! does not let them act on the call.
//!
//! The secret is read from `REALTIME_CALLID_HMAC_KEY`; if unset, it falls
//! back to `DATABASE_SECRET_KEY` so deployments that already configure the
//! database secret get this for free.

use std::sync::OnceLock;

use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use thiserror::Error;

const WRAPPED_PREFIX: &str = "rtc_";
const RAW_PREFIX: &str = "rtc_";
const SIG_HEX_LEN: usize = 16;

const PRIMARY_ENV: &str = "REALTIME_CALLID_HMAC_KEY";
const FALLBACK_ENV: &str = "DATABASE_SECRET_KEY";

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CallIdError {
    #[error("wrapped call_id missing rtc_ prefix")]
    MissingPrefix,
    #[error("wrapped call_id has wrong shape")]
    MalformedShape,
    #[error("wrapped call_id user_id is not an integer")]
    InvalidUserId,
    #[error("wrapped call_id does not belong to the authenticated user")]
    UserMismatch,
    #[error("wrapped call_id signature is invalid")]
    SignatureMismatch,
    #[error("REALTIME_CALLID_HMAC_KEY is not configured")]
    SecretUnavailable,
}

fn load_secret() -> Option<Vec<u8>> {
    if let Ok(value) = std::env::var(PRIMARY_ENV)
        && !value.trim().is_empty()
    {
        return Some(value.into_bytes());
    }
    if let Ok(value) = std::env::var(FALLBACK_ENV)
        && !value.trim().is_empty()
    {
        return Some(value.into_bytes());
    }
    None
}

fn secret() -> Option<&'static [u8]> {
    static CACHED: OnceLock<Option<Vec<u8>>> = OnceLock::new();
    CACHED.get_or_init(load_secret).as_deref()
}

fn compute_signature(secret: &[u8], user_id: i64, real_call_id: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(secret);
    hasher.update(b"|");
    hasher.update(user_id.to_string().as_bytes());
    hasher.update(b"|");
    hasher.update(real_call_id.as_bytes());
    let digest = hasher.finalize();
    let mut out = [0u8; 8];
    out.copy_from_slice(&digest[..8]);
    out
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

/// Wrap an upstream `call_id` for delivery to the given downstream user.
///
/// Returns the `SignatureUnavailable` error when no secret is configured.
pub fn wrap(user_id: i64, real_call_id: &str) -> Result<String, CallIdError> {
    let secret = secret().ok_or(CallIdError::SecretUnavailable)?;
    let tail = real_call_id.strip_prefix(RAW_PREFIX).unwrap_or(real_call_id);
    let sig = compute_signature(secret, user_id, real_call_id);
    Ok(format!(
        "{WRAPPED_PREFIX}{user_id}_{}_{tail}",
        hex_encode(&sig)
    ))
}

/// Unwrap a wrapped `call_id` and return the upstream real id.
///
/// Verifies the signature and that the embedded `user_id` matches the
/// authenticated user.
pub fn unwrap(wrapped: &str, expected_user_id: i64) -> Result<String, CallIdError> {
    let secret = secret().ok_or(CallIdError::SecretUnavailable)?;
    let body = wrapped
        .strip_prefix(WRAPPED_PREFIX)
        .ok_or(CallIdError::MissingPrefix)?;

    let mut parts = body.splitn(3, '_');
    let user_id_str = parts.next().ok_or(CallIdError::MalformedShape)?;
    let sig_hex = parts.next().ok_or(CallIdError::MalformedShape)?;
    let tail = parts.next().ok_or(CallIdError::MalformedShape)?;

    if sig_hex.len() != SIG_HEX_LEN {
        return Err(CallIdError::MalformedShape);
    }
    let embedded_user_id: i64 = user_id_str.parse().map_err(|_| CallIdError::InvalidUserId)?;
    if embedded_user_id != expected_user_id {
        return Err(CallIdError::UserMismatch);
    }

    let real_call_id = format!("{RAW_PREFIX}{tail}");
    let expected_sig = compute_signature(secret, expected_user_id, &real_call_id);
    let expected_sig_hex = hex_encode(&expected_sig);

    if expected_sig_hex.as_bytes().ct_eq(sig_hex.as_bytes()).unwrap_u8() != 1 {
        return Err(CallIdError::SignatureMismatch);
    }

    Ok(real_call_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_secret<T>(value: &str, f: impl FnOnce() -> T) -> T {
        unsafe {
            std::env::set_var(PRIMARY_ENV, value);
        }
        // OnceLock means the secret is cached for the process lifetime, so
        // only the first `with_secret` call's value takes effect. Tests that
        // need the same secret are fine; tests with distinct secrets must
        // live in separate binaries.
        f()
    }

    #[test]
    fn wrap_unwrap_roundtrip() {
        with_secret("test-secret-xyz", || {
            let wrapped = wrap(42, "rtc_abc123").expect("wrap");
            assert!(wrapped.starts_with("rtc_42_"));
            assert!(wrapped.ends_with("_abc123"));
            let real = unwrap(&wrapped, 42).expect("unwrap");
            assert_eq!(real, "rtc_abc123");
        });
    }

    #[test]
    fn wrong_user_rejected() {
        with_secret("test-secret-xyz", || {
            let wrapped = wrap(42, "rtc_abc123").expect("wrap");
            assert_eq!(unwrap(&wrapped, 99), Err(CallIdError::UserMismatch));
        });
    }

    #[test]
    fn tampered_signature_rejected() {
        with_secret("test-secret-xyz", || {
            let wrapped = wrap(42, "rtc_abc123").expect("wrap");
            let mut bytes: Vec<u8> = wrapped.into_bytes();
            let sig_start = b"rtc_42_".len();
            bytes[sig_start] ^= 0x01;
            let tampered = String::from_utf8(bytes).unwrap();
            assert_eq!(unwrap(&tampered, 42), Err(CallIdError::SignatureMismatch));
        });
    }

    #[test]
    fn missing_prefix_rejected() {
        with_secret("test-secret-xyz", || {
            assert_eq!(
                unwrap("gp_42_aaaaaaaaaaaaaaaa_abc", 42),
                Err(CallIdError::MissingPrefix)
            );
        });
    }

    #[test]
    fn malformed_shape_rejected() {
        with_secret("test-secret-xyz", || {
            assert_eq!(unwrap("rtc_42_short_abc", 42), Err(CallIdError::MalformedShape));
            assert_eq!(unwrap("rtc_onlyonepart", 42), Err(CallIdError::MalformedShape));
        });
    }
}
