use http::StatusCode;
use serde::{Deserialize, Serialize};

pub use crate::openai::realtime_calls::types::ResponseBody;
use crate::openai::realtime_calls::types::{OpenAiApiErrorResponse, OpenAiResponseHeaders};

/// Full HTTP response for `POST /v1/realtime/calls/{call_id}/accept`.
///
/// On success the server returns `200 OK` with an empty or minimal JSON
/// object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiRealtimeCallAcceptResponse {
    Success {
        #[serde(with = "crate::openai::types::status_code_serde")]
        stats_code: StatusCode,
        headers: OpenAiResponseHeaders,
        body: ResponseBody,
    },
    Error {
        #[serde(with = "crate::openai::types::status_code_serde")]
        stats_code: StatusCode,
        headers: OpenAiResponseHeaders,
        body: OpenAiApiErrorResponse,
    },
}
