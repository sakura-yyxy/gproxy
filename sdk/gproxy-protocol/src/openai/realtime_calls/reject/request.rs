use serde::{Deserialize, Serialize};

use crate::openai::realtime_calls::types::{
    HttpMethod, PathParameters, QueryParameters, RequestHeaders,
};

/// Request descriptor for `POST /v1/realtime/calls/{call_id}/reject`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenAiRealtimeCallRejectRequest {
    pub method: HttpMethod,
    pub path: PathParameters,
    pub query: QueryParameters,
    pub headers: RequestHeaders,
    pub body: RequestBody,
}

impl Default for OpenAiRealtimeCallRejectRequest {
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

/// Body payload for the reject endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestBody {
    /// SIP response code to send back to the caller. Defaults to `603`
    /// (Decline) when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<u16>,
}
