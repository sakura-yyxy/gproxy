//! Body-level types for `POST /v1/realtime/client_secrets`.

use serde::{Deserialize, Serialize};

use crate::openai::create_response::types::JsonObject;
use crate::openai::realtime::types::{
    RealtimeAudioFormat, RealtimeInputAudioTranscription, RealtimeNoiseReduction, RealtimeSession,
    RealtimeTurnDetection,
};

/// `expires_after` object: when a client secret stops being usable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ExpiresAfter {
    /// Anchor point; currently only `"created_at"` is supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<ExpiresAfterAnchor>,
    /// Seconds from the anchor (10..=7200, default 600).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<u32>,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Anchor enum for [`ExpiresAfter`]; only `created_at` is defined.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ExpiresAfterAnchor {
    #[default]
    #[serde(rename = "created_at")]
    CreatedAt,
}

/// Session config attached to the client secret, tagged by the session `type` field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ClientSecretSessionConfig {
    /// Full Realtime session configuration (`type: "realtime"`).
    Realtime(RealtimeSession),
    /// Transcription-only session configuration (`type: "transcription"`).
    Transcription(RealtimeTranscriptionSession),
}

/// Transcription-only session configuration (`type: "transcription"`).
///
/// Narrower shape than [`RealtimeSession`] — only `audio` (input side) and
/// `include` are defined for transcription sessions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeTranscriptionSession {
    /// Server-assigned session id (`sess_...`), present on responses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Object identifier, typically `realtime.transcription_session` on responses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// Unix timestamp for session expiry (response only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    /// Ephemeral client secret returned on the response side.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<RealtimeSessionClientSecret>,
    /// Input-audio configuration (format / noise reduction / transcription / turn detection).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<RealtimeTranscriptionSessionAudio>,
    /// Include-list of additional fields in server outputs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// Any other fields the server includes.
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Transcription session `audio` block (`{ input }`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeTranscriptionSessionAudio {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<RealtimeTranscriptionSessionAudioInput>,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Transcription session input-audio configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeTranscriptionSessionAudioInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<RealtimeAudioFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noise_reduction: Option<RealtimeNoiseReduction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcription: Option<RealtimeInputAudioTranscription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeTurnDetection>,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Ephemeral client secret sub-object nested inside a session response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RealtimeSessionClientSecret {
    /// Ephemeral key (e.g. `ek_...`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Unix timestamp when the ephemeral key expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}
