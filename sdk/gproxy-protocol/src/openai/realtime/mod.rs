//! OpenAI Realtime WebSocket API wire types (`GET /v1/realtime`).
//!
//! - [`request`] — connect descriptor (method / path / query / headers / optional first frame).
//! - [`types`] — shared types (session, conversation items, audio formats, turn detection, usage, rate limits, errors).
//! - [`client_events`] — 11-variant [`client_events::OpenAiRealtimeClientEvent`] union.
//! - [`server_events`] — [`server_events::OpenAiRealtimeServerEvent`] union of all documented server events.

pub mod client_events;
pub mod request;
pub mod server_events;
pub mod types;
