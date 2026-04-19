//! Request descriptor for `POST /v1/realtime/client_secrets`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::openai::create_client_secret::types::{ClientSecretSessionConfig, ExpiresAfter};
use crate::openai::create_response::types::HttpMethod;

/// Request descriptor for OpenAI `realtime.client_secrets.create` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateClientSecretRequest {
    /// HTTP method.
    pub method: HttpMethod,
    /// Path parameters.
    pub path: PathParameters,
    /// Query parameters.
    pub query: QueryParameters,
    /// Request headers.
    pub headers: RequestHeaders,
    /// Request body.
    pub body: RequestBody,
}

impl Default for OpenAiCreateClientSecretRequest {
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

/// `realtime.client_secrets.create` does not define path params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PathParameters {}

/// `realtime.client_secrets.create` does not define query params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {}

/// Proxy-side request model does not carry auth headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Body payload for `POST /v1/realtime/client_secrets`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestBody {
    /// Configuration for the client secret expiration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<ExpiresAfter>,
    /// Session configuration to use for the client secret (realtime or transcription).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<ClientSecretSessionConfig>,
}
