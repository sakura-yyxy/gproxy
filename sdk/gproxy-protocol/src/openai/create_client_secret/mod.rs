//! `POST /v1/realtime/client_secrets` wire types.
//!
//! - [`request`] — request descriptor (method/path/query/headers/body)
//! - [`response`] — successful + error response descriptor
//! - [`types`] — body-level types (`ExpiresAfter`, session config unions,
//!   `RealtimeTranscriptionSession`)

pub mod request;
pub mod response;
pub mod types;
