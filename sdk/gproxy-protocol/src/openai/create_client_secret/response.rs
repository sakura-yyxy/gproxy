//! Response descriptor for `POST /v1/realtime/client_secrets`.

use std::collections::BTreeMap;

use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::openai::create_client_secret::types::ClientSecretSessionConfig;
use crate::openai::create_response::types::OpenAiApiErrorResponse;

/// Successful body returned by `realtime.client_secrets.create`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseBody {
    /// Ephemeral client-secret string (e.g. `ek_1234`).
    pub value: String,
    /// Unix timestamp (seconds since epoch) when the secret expires.
    pub expires_at: i64,
    /// Effective session configuration associated with the secret.
    pub session: ClientSecretSessionConfig,
}

/// Response headers for `realtime.client_secrets.create`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OpenAiCreateClientSecretResponseHeaders {
    /// `x-request-id` returned by OpenAI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Additional response headers.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Full HTTP response for OpenAI `realtime.client_secrets.create` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiCreateClientSecretResponse {
    Success {
        /// HTTP status code returned by server (should be `200 OK`).
        #[serde(with = "crate::openai::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: OpenAiCreateClientSecretResponseHeaders,
        /// Successful body.
        body: ResponseBody,
    },
    Error {
        /// HTTP status code returned by server (typically non-2xx).
        #[serde(with = "crate::openai::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: OpenAiCreateClientSecretResponseHeaders,
        /// Error body.
        body: OpenAiApiErrorResponse,
    },
}
