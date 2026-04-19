use std::collections::BTreeMap;

use crate::routing::RouteKey;

/// A prepared upstream request, protocol-agnostic.
#[derive(Debug, Clone)]
pub struct PreparedRequest {
    /// HTTP method.
    pub method: http::Method,
    /// Semantic upstream route (operation + protocol).
    pub route: RouteKey,
    /// Target model name (if known).
    pub model: Option<String>,
    /// Request body bytes.
    pub body: Vec<u8>,
    /// Extra headers to forward.
    pub headers: http::HeaderMap,
    /// Path parameters captured from the inbound URL (e.g. `call_id` for
    /// `/realtime/calls/{call_id}/accept`). Empty for endpoints whose URL has
    /// no templated segments.
    pub path_params: BTreeMap<String, String>,
}
