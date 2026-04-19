//! Request descriptor for the OpenAI Realtime WebSocket endpoint
//! (`GET /v1/realtime`).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::client_events::OpenAiRealtimeClientEvent;
use crate::openai::create_response::types::HttpMethod;

/// Request descriptor for the OpenAI Realtime WebSocket upgrade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeConnectRequest {
    /// HTTP method used for the WebSocket upgrade handshake.
    pub method: HttpMethod,
    /// Path selector (`/realtime`).
    pub path: PathParameters,
    /// Query parameters (e.g. `model`).
    pub query: QueryParameters,
    /// Request headers (auth, `OpenAI-Beta`, extras).
    pub headers: RequestHeaders,
    /// Optional first WebSocket frame to send after connect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<RequestBody>,
}

impl Default for OpenAiRealtimeConnectRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::Get,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: None,
        }
    }
}

/// Path selector for the realtime endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PathParameters {
    /// WebSocket route under the provider base URL.
    #[serde(default)]
    pub endpoint: OpenAiRealtimeEndpoint,
}

/// Realtime endpoint selector. Currently only `/realtime` is documented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OpenAiRealtimeEndpoint {
    #[default]
    #[serde(rename = "realtime")]
    Realtime,
}

/// Query parameters on the realtime handshake.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {
    /// Realtime model to open the session with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Azure-compatible API version query key.
    #[serde(
        rename = "api-version",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    /// Provider-specific passthrough query params.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Headers commonly used with the realtime endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(
        rename = "Authorization",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization: Option<String>,
    #[serde(
        rename = "OpenAI-Beta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub openai_beta: Option<String>,
    /// Provider-specific passthrough headers.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Optional initial WebSocket frame sent right after connection.
pub type RequestBody = OpenAiRealtimeClientEvent;
