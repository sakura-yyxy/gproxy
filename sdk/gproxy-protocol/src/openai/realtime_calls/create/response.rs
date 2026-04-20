use std::collections::BTreeMap;

use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::openai::realtime_calls::types::{JsonObject, OpenAiApiErrorResponse};

/// Full HTTP response for `POST /v1/realtime/calls` (OpenAI direct) and
/// `POST /realtime/calls` (Codex backend).
///
/// On success the server returns `201 Created` with an `application/sdp`
/// body (the SDP answer) and a `Location: /v1/realtime/calls/{call_id}`
/// header whose final path segment (after stripping any query string) is
/// the newly-created `rtc_*` call_id. The proxy forwards the raw body
/// bytes and headers as-is; this typed model is for documentation + SDK
/// use only.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiRealtimeCallCreateResponse {
    Success {
        #[serde(with = "crate::openai::types::status_code_serde")]
        stats_code: StatusCode,
        headers: ResponseHeaders,
        body: ResponseBody,
    },
    Error {
        #[serde(with = "crate::openai::types::status_code_serde")]
        stats_code: StatusCode,
        headers: ResponseHeaders,
        body: OpenAiApiErrorResponse,
    },
}

/// Response headers for the create-call endpoint.
///
/// The `Location` header carries the new call resource path
/// (`/v1/realtime/calls/{call_id}`); its final segment (with any `?query`
/// stripped) matches the `rtc_*` call_id assigned by the server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ResponseHeaders {
    #[serde(rename = "Location", default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(
        rename = "x-request-id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_id: Option<String>,
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Successful response body — the upstream returns raw `application/sdp`
/// text here, **not** JSON. The SDP answer lives in the HTTP body bytes;
/// no JSON fields are defined. This struct is an empty forward-compat
/// container preserved to match the sibling wire-type shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ResponseBody {
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}
