//! Shared wire types for the OpenAI Realtime SIP call control endpoints:
//! `POST /v1/realtime/calls/{call_id}/{accept,hangup,refer,reject}`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use crate::openai::create_response::types::{
    HttpMethod, JsonObject, OpenAiApiErrorResponse, OpenAiResponseHeaders,
};

/// Path parameters shared by all four realtime-call endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PathParameters {
    /// Server-assigned call ID (`call_...`).
    pub call_id: String,
}

/// Query parameters — none of the four endpoints define query params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {}

/// Proxy-side request model does not carry auth headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Successful response body — these endpoints return `200 OK` with an empty
/// or minimal JSON object. Forward-compat extra fields are captured.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ResponseBody {
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}
