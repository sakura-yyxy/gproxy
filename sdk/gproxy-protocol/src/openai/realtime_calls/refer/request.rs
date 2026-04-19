use serde::{Deserialize, Serialize};

use crate::openai::realtime_calls::types::{HttpMethod, PathParameters, QueryParameters, RequestHeaders};

/// Request descriptor for `POST /v1/realtime/calls/{call_id}/refer`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenAiRealtimeCallReferRequest {
    pub method: HttpMethod,
    pub path: PathParameters,
    pub query: QueryParameters,
    pub headers: RequestHeaders,
    pub body: RequestBody,
}

impl Default for OpenAiRealtimeCallReferRequest {
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

/// Body payload for the refer endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestBody {
    /// URI that should appear in the SIP `Refer-To` header. Supports values
    /// like `tel:+14155550123` or `sip:agent@example.com`.
    pub target_uri: String,
}
