use serde::{Deserialize, Serialize};

use crate::openai::realtime::types::RealtimeSession;
use crate::openai::realtime_calls::types::{HttpMethod, PathParameters, QueryParameters, RequestHeaders};

/// Request descriptor for `POST /v1/realtime/calls/{call_id}/accept`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeCallAcceptRequest {
    /// HTTP method.
    pub method: HttpMethod,
    /// Path parameters (`call_id`).
    pub path: PathParameters,
    /// Query parameters (none).
    pub query: QueryParameters,
    /// Request headers.
    pub headers: RequestHeaders,
    /// Request body — a full realtime session config (same shape as
    /// `session.update`), including the `type: "realtime"` discriminator on
    /// [`RealtimeSession::type_`].
    pub body: RequestBody,
}

impl Default for OpenAiRealtimeCallAcceptRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::Post,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        }
    }
}

/// Body payload for the accept endpoint — the full realtime session config.
///
/// Reuses [`RealtimeSession`] verbatim. The `type: "realtime"` discriminator
/// required by the OpenAI docs is carried on [`RealtimeSession::type_`].
pub type RequestBody = RealtimeSession;
