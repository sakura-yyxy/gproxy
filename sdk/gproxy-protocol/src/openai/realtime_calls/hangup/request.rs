use serde::{Deserialize, Serialize};

use crate::openai::realtime_calls::types::{
    HttpMethod, PathParameters, QueryParameters, RequestHeaders,
};

/// Request descriptor for `POST /v1/realtime/calls/{call_id}/hangup`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenAiRealtimeCallHangupRequest {
    pub method: HttpMethod,
    pub path: PathParameters,
    pub query: QueryParameters,
    pub headers: RequestHeaders,
    pub body: RequestBody,
}

impl Default for OpenAiRealtimeCallHangupRequest {
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

/// Empty request body — hangup takes no parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestBody {}
