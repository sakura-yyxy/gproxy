//! OpenAI Realtime SIP call control endpoints.
//!
//! - [`create`] — `POST /v1/realtime/calls` (OpenAI direct) / `POST /realtime/calls` (Codex backend)
//! - [`accept`] — `POST /v1/realtime/calls/{call_id}/accept`
//! - [`hangup`] — `POST /v1/realtime/calls/{call_id}/hangup`
//! - [`refer`]  — `POST /v1/realtime/calls/{call_id}/refer`
//! - [`reject`] — `POST /v1/realtime/calls/{call_id}/reject`
//! - [`types`]  — shared path/header/response types for all four endpoints

pub mod types;

pub mod accept;
pub mod create;
pub mod hangup;
pub mod refer;
pub mod reject;
