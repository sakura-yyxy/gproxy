use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::openai::realtime::types::RealtimeSession;
use crate::openai::realtime_calls::types::{HttpMethod, JsonObject, RequestHeaders};

/// Request descriptor for `POST /v1/realtime/calls` (OpenAI direct) and
/// `POST /realtime/calls` (Codex backend).
///
/// The proxy forwards the HTTP body bytes opaquely — the typed
/// [`RequestBody`] below models the Codex-backend JSON variant for
/// documentation and SDK use only. The OpenAI-direct multipart and raw
/// `application/sdp` variants are transport concerns handled at the HTTP
/// layer, not here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeCallCreateRequest {
    /// HTTP method.
    pub method: HttpMethod,
    /// Path parameters — none (the endpoint path has no templated segments
    /// at create time; the `call_id` is assigned server-side and surfaced via
    /// the response `Location` header).
    pub path: PathParameters,
    /// Query parameters — none defined by the endpoint.
    pub query: QueryParameters,
    /// Request headers.
    pub headers: RequestHeaders,
    /// Request body — see [`RequestBody`].
    pub body: RequestBody,
}

impl Default for OpenAiRealtimeCallCreateRequest {
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

/// Path parameters — empty for the create endpoint (no templated segments).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PathParameters {}

/// Query parameters — none defined; captured for forward compat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Body payload — models the **Codex-backend JSON variant** of the create
/// call request: `{ "sdp": "<offer>", "session": {...} }`.
///
/// The OpenAI-direct endpoint accepts either a `multipart/form-data` body
/// (with `sdp` + `session` parts) or a raw `application/sdp` body. Those
/// transport shapes are intentionally NOT modeled here — the proxy layer
/// forwards those body bytes without parsing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestBody {
    /// SDP offer from the client.
    pub sdp: String,
    /// Optional realtime session configuration (mirrors `session.update`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<RealtimeSession>,
    /// Forward-compat extra fields.
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}
