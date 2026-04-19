//! Shared wire types for the OpenAI Realtime WebSocket API (`/v1/realtime`).
//!
//! These types model the objects that are embedded inside client and server
//! events (session configuration, conversation items, content parts, audio
//! formats, turn detection, tools, usage, rate limits, and the error shape).

use serde::{Deserialize, Serialize};

pub use crate::openai::create_response::types::{HttpMethod, JsonObject, Metadata};

/// Free-form conversation role string (`system`, `user`, `assistant`).
pub type RealtimeRole = String;

/// Audio formats accepted by the Realtime API input/output paths.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RealtimeAudioFormat {
    /// PCM 16-bit @ 24kHz.
    #[serde(rename = "audio/pcm")]
    Pcm {
        /// Sample rate. Always `24000`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        rate: Option<u32>,
    },
    /// G.711 μ-law.
    #[serde(rename = "audio/pcmu")]
    Pcmu {},
    /// G.711 A-law.
    #[serde(rename = "audio/pcma")]
    Pcma {},
}

/// Output modality for Realtime responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeOutputModality {
    Text,
    Audio,
}

/// Built-in voice identifiers documented by the Realtime API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeVoice {
    /// A documented voice name or an ID object `{ "id": "voice_..." }`.
    Named(RealtimeNamedVoice),
    /// Custom voice object wrapper.
    Custom(RealtimeCustomVoice),
    /// Free-form voice identifier (forward-compatible fallback).
    Other(String),
}

/// Documented built-in voice names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeNamedVoice {
    Alloy,
    Ash,
    Ballad,
    Coral,
    Echo,
    Sage,
    Shimmer,
    Verse,
    Marin,
    Cedar,
}

/// Custom voice reference `{ "id": "voice_1234" }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeCustomVoice {
    pub id: String,
}

/// Noise reduction type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeNoiseReductionType {
    NearField,
    FarField,
}

/// Session-level noise reduction configuration wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RealtimeNoiseReduction {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<RealtimeNoiseReductionType>,
}

/// Turn-detection configuration (server VAD or semantic VAD).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RealtimeTurnDetection {
    /// Server-side voice activity detection.
    #[serde(rename = "server_vad")]
    ServerVad {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        create_response: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        idle_timeout_ms: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interrupt_response: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prefix_padding_ms: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        silence_duration_ms: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        threshold: Option<f64>,
    },
    /// Semantic VAD using a turn-detection model.
    #[serde(rename = "semantic_vad")]
    SemanticVad {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        create_response: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        eagerness: Option<RealtimeSemanticVadEagerness>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interrupt_response: Option<bool>,
    },
}

/// Semantic VAD eagerness setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSemanticVadEagerness {
    Low,
    Medium,
    High,
    Auto,
}

/// Status of a realtime conversation item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeItemStatus {
    Completed,
    Incomplete,
    InProgress,
}

/// A content part within a realtime `message` item.
///
/// Unions the union of fields across system/user/assistant content parts as
/// documented — individual content part `type`s only populate a subset of
/// fields (e.g. `input_text` uses `text`, `input_audio` uses `audio`/`transcript`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimeMessageContentPart {
    /// Content part kind: `input_text`, `input_audio`, `input_image`,
    /// `output_text`, or `output_audio`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Text for `input_text` / `output_text` parts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Base64-encoded audio bytes (input or output audio parts).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// Transcript accompanying an audio part.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
    /// Image detail for `input_image` parts: `auto` / `low` / `high`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// Data URL containing base64-encoded image bytes for `input_image` parts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// Content part inside a streaming `response.content_part.*` event
/// (a subset of [`RealtimeMessageContentPart`] with `audio` / `text` types only).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimeStreamingContentPart {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
}

/// Tool definition used on the MCP list-tools item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimeMcpListedTool {
    pub name: String,
    pub input_schema: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Error payload attached to an `mcp_call` item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RealtimeMcpToolError {
    #[serde(rename = "protocol_error")]
    Protocol { code: i64, message: String },
    #[serde(rename = "tool_execution_error")]
    ToolExecution { message: String },
    #[serde(rename = "http_error")]
    Http { code: i64, message: String },
}

/// A single item within a Realtime conversation.
///
/// Keyed by the item `type` field. The `message` variant stores the role
/// separately since the doc differentiates system/user/assistant messages by
/// role. Other variants map to their documented item shapes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RealtimeConversationItem {
    /// `type: "message"` — system / user / assistant message.
    #[serde(rename = "message")]
    Message(RealtimeMessageItem),
    /// `type: "function_call"` — assistant function call.
    #[serde(rename = "function_call")]
    FunctionCall(RealtimeFunctionCallItem),
    /// `type: "function_call_output"` — client-supplied function result.
    #[serde(rename = "function_call_output")]
    FunctionCallOutput(RealtimeFunctionCallOutputItem),
    /// `type: "mcp_approval_response"`.
    #[serde(rename = "mcp_approval_response")]
    McpApprovalResponse(RealtimeMcpApprovalResponseItem),
    /// `type: "mcp_list_tools"`.
    #[serde(rename = "mcp_list_tools")]
    McpListTools(RealtimeMcpListToolsItem),
    /// `type: "mcp_call"`.
    #[serde(rename = "mcp_call")]
    McpCall(RealtimeMcpCallItem),
    /// `type: "mcp_approval_request"`.
    #[serde(rename = "mcp_approval_request")]
    McpApprovalRequest(RealtimeMcpApprovalRequestItem),
}

/// A `message` conversation item (system/user/assistant).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimeMessageItem {
    pub role: RealtimeRole,
    pub content: Vec<RealtimeMessageContentPart>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RealtimeItemStatus>,
}

/// A `function_call` conversation item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimeFunctionCallItem {
    pub arguments: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RealtimeItemStatus>,
}

/// A `function_call_output` conversation item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimeFunctionCallOutputItem {
    pub call_id: String,
    pub output: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RealtimeItemStatus>,
}

/// An `mcp_approval_response` conversation item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimeMcpApprovalResponseItem {
    pub id: String,
    pub approval_request_id: String,
    pub approve: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// An `mcp_list_tools` conversation item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimeMcpListToolsItem {
    pub server_label: String,
    pub tools: Vec<RealtimeMcpListedTool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// An `mcp_call` conversation item (tool invocation).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimeMcpCallItem {
    pub id: String,
    pub arguments: String,
    pub name: String,
    pub server_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_request_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<RealtimeMcpToolError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// An `mcp_approval_request` conversation item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimeMcpApprovalRequestItem {
    pub id: String,
    pub arguments: String,
    pub name: String,
    pub server_label: String,
}

/// Realtime session configuration as delivered by `session.created` /
/// `session.updated` and accepted by `session.update`.
///
/// The Realtime session schema has a very large surface area (audio I/O,
/// tracing, tools, prompts, truncation, MCP configuration, etc.) and is
/// forward-compatible. We model the most commonly observed top-level keys
/// explicitly and retain all remaining fields via [`JsonObject`] `extra`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeSession {
    /// Session kind: `realtime` or `transcription`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Object identifier, always `realtime.session` when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// Server-assigned session ID (`sess_...`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Model identifier (`gpt-realtime`, etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Output modalities the model may respond with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<RealtimeOutputModality>>,
    /// System instructions string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Tools configuration (free-form: function / MCP / etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    /// Tool-choice configuration (string or object).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<serde_json::Value>,
    /// Maximum output tokens per response (`number` or `"inf"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<serde_json::Value>,
    /// Tracing configuration (`auto` or object).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<serde_json::Value>,
    /// Prompt template reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<serde_json::Value>,
    /// Unix timestamp for session expiry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    /// Audio input / output configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<serde_json::Value>,
    /// Include-list of additional fields in server outputs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// Truncation policy (`auto` / `disabled` / object).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncation: Option<serde_json::Value>,
    /// Any other session fields the server includes.
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Final status of a realtime response.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeResponseStatus {
    Completed,
    Cancelled,
    Failed,
    Incomplete,
    InProgress,
}

/// Status-detail reason code for non-completed responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeResponseStatusReason {
    TurnDetected,
    ClientCancelled,
    MaxOutputTokens,
    ContentFilter,
}

/// Status-detail type discriminator (mirrors the response `status` field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeResponseStatusType {
    Completed,
    Cancelled,
    Failed,
    Incomplete,
}

/// `status_details` shape on a realtime response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeResponseStatusDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<RealtimeResponseStatusError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<RealtimeResponseStatusReason>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<RealtimeResponseStatusType>,
}

/// Inner error shape nested inside `status_details.error`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeResponseStatusError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// Cached-token breakdown inside input-token usage details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeUsageCachedTokensDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_tokens: Option<u64>,
}

/// Details breaking down input tokens for a realtime response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeUsageInputTokenDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cached_tokens_details: Option<RealtimeUsageCachedTokensDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_tokens: Option<u64>,
}

/// Details breaking down output tokens for a realtime response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeUsageOutputTokenDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_tokens: Option<u64>,
}

/// Per-response billing usage object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeResponseUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_token_details: Option<RealtimeUsageInputTokenDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_token_details: Option<RealtimeUsageOutputTokenDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<u64>,
}

/// Audio output configuration emitted on a realtime response object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeResponseAudioOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<RealtimeAudioFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<RealtimeVoice>,
}

/// `audio` block on a realtime response resource.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeResponseAudio {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<RealtimeResponseAudioOutput>,
}

/// Realtime response resource as delivered on `response.created` /
/// `response.done` events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<RealtimeResponseAudio>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// `number` or the string `"inf"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<RealtimeConversationItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<RealtimeOutputModality>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RealtimeResponseStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<RealtimeResponseStatusDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<RealtimeResponseUsage>,
    /// Forward-compatible passthrough for any additional fields.
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Logprob entry for input-audio transcription events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimeLogProb {
    pub token: String,
    pub bytes: Vec<i64>,
    pub logprob: f64,
}

/// Usage reported by the transcription ASR pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RealtimeTranscriptionUsage {
    /// Token-based billing.
    #[serde(rename = "tokens")]
    Tokens {
        input_tokens: u64,
        output_tokens: u64,
        total_tokens: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        input_token_details: Option<RealtimeTranscriptionTokenDetails>,
    },
    /// Duration-based billing.
    #[serde(rename = "duration")]
    Duration { seconds: f64 },
}

/// Token breakdown returned with transcription usage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeTranscriptionTokenDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_tokens: Option<u64>,
}

/// Rate-limit entry emitted on `rate_limits.updated`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeRateLimit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<RealtimeRateLimitName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset_seconds: Option<f64>,
}

/// Documented rate-limit window identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeRateLimitName {
    Requests,
    Tokens,
}

/// Error payload delivered on a realtime `error` server event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeError {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    /// Forward-compatible passthrough.
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Transcription failure error wrapper used on
/// `conversation.item.input_audio_transcription.failed`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeTranscriptionError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// Realtime `conversation` resource reported on `conversation.created`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeConversationResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}
