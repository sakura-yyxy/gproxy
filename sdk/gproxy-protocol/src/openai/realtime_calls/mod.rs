//! OpenAI Realtime SIP call control endpoints.
//!
//! - [`accept`] — `POST /v1/realtime/calls/{call_id}/accept`
//! - [`hangup`] — `POST /v1/realtime/calls/{call_id}/hangup`
//! - [`refer`]  — `POST /v1/realtime/calls/{call_id}/refer`
//! - [`reject`] — `POST /v1/realtime/calls/{call_id}/reject`
//! - [`types`]  — shared path/header/response types for all four endpoints

pub mod types;

pub mod accept;
pub mod hangup;
pub mod refer;
pub mod reject;
