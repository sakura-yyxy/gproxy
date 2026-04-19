//! Shared wire types for the OpenAI Realtime WebSocket API (`/v1/realtime`).
//!
//! These types model the objects that are embedded inside client and server
//! events (session configuration, conversation items, content parts, audio
//! formats, turn detection, tools, usage, rate limits, and the error shape).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use crate::openai::count_tokens::types::{
    ResponseInputFile, ResponseInputImage, ResponseInputText,
};
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

/// Image detail level for `input_image` content parts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeImageDetail {
    Auto,
    Low,
    High,
}

/// A content part within a realtime `message` item, keyed by `type`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RealtimeMessageContentPart {
    /// `input_text` — plain text input from system/user messages.
    #[serde(rename = "input_text")]
    InputText {
        text: String,
    },
    /// `input_audio` — base64 audio bytes with optional transcript (user messages).
    #[serde(rename = "input_audio")]
    InputAudio {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        audio: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transcript: Option<String>,
    },
    /// `input_image` — base64 image bytes as a data URI (user messages).
    #[serde(rename = "input_image")]
    InputImage {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        detail: Option<RealtimeImageDetail>,
    },
    /// `output_text` — plain text output (assistant messages).
    #[serde(rename = "output_text")]
    OutputText {
        text: String,
    },
    /// `output_audio` — base64 audio bytes with transcript (assistant messages).
    #[serde(rename = "output_audio")]
    OutputAudio {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        audio: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transcript: Option<String>,
    },
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
    /// `type: "item_reference"` — reference to an existing item by id
    /// (used in `response.create.input`).
    #[serde(rename = "item_reference")]
    ItemReference(RealtimeItemReference),
}

/// An `item_reference` — references an existing conversation item by id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeItemReference {
    pub id: String,
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
    /// Tools configuration (function / MCP tools).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<RealtimeTool>>,
    /// Tool-choice configuration (string mode or forced function/MCP tool).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<RealtimeToolChoice>,
    /// Maximum output tokens per response (`number` or `"inf"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<RealtimeMaxOutputTokens>,
    /// Tracing configuration (`auto` or object).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<RealtimeTracing>,
    /// Prompt template reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<RealtimePromptRef>,
    /// Unix timestamp for session expiry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    /// Audio input / output configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<RealtimeAudioConfig>,
    /// Include-list of additional fields in server outputs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// Truncation policy (`auto` / `disabled` / retention-ratio object).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncation: Option<RealtimeTruncation>,
    /// Any other session fields the server includes.
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Top-level `audio` block on a session (`{ input, output }`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeAudioConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<RealtimeAudioConfigInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<RealtimeAudioConfigOutput>,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Session input-audio configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeAudioConfigInput {
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

/// Session output-audio configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeAudioConfigOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<RealtimeAudioFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<RealtimeVoice>,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Input-audio ASR transcription configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeInputAudioTranscription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Tools configuration entry — a function tool or an MCP tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RealtimeTool {
    /// Function tool (`type: "function"`).
    #[serde(rename = "function")]
    Function(RealtimeFunctionTool),
    /// Remote MCP tool (`type: "mcp"`).
    #[serde(rename = "mcp")]
    Mcp(RealtimeMcpTool),
}

/// Function tool definition used on the Realtime session `tools` array.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeFunctionTool {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// MCP tool definition for a session (remote MCP server).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimeMcpTool {
    pub server_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<RealtimeMcpAllowedTools>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defer_loading: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_approval: Option<RealtimeMcpApprovalConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// `allowed_tools` for an MCP tool — either a list of tool names or a filter object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeMcpAllowedTools {
    /// Plain list of tool names.
    Names(Vec<String>),
    /// Filter object (read-only flag plus name list).
    Filter(RealtimeMcpToolFilter),
}

/// Filter object for MCP `allowed_tools` / approval entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RealtimeMcpToolFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_names: Option<Vec<String>>,
}

/// `require_approval` on an MCP tool — either a setting literal or a filter object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeMcpApprovalConfig {
    /// Filter object `{ always?, never? }`.
    Filter(RealtimeMcpApprovalFilter),
    /// Single approval setting (`"always"` or `"never"`).
    Setting(RealtimeMcpApprovalSetting),
}

/// MCP approval filter with optional `always` / `never` inclusion lists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RealtimeMcpApprovalFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub always: Option<RealtimeMcpToolFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub never: Option<RealtimeMcpToolFilter>,
}

/// MCP approval setting literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeMcpApprovalSetting {
    Always,
    Never,
}

/// Tool-choice configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeToolChoice {
    /// Named mode string (`none` / `auto` / `required`).
    Mode(RealtimeToolChoiceMode),
    /// Force a specific function call (`{type:"function", name}`).
    Function(RealtimeToolChoiceFunction),
    /// Force a specific MCP tool call (`{type:"mcp", server_label, name?}`).
    Mcp(RealtimeToolChoiceMcp),
}

/// Tool-choice named-mode string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeToolChoiceMode {
    None,
    Auto,
    Required,
}

/// Force-call-a-function tool choice variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeToolChoiceFunction {
    #[serde(rename = "type")]
    pub type_: RealtimeToolChoiceFunctionType,
    pub name: String,
}

/// Marker type for `{"type":"function"}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeToolChoiceFunctionType {
    Function,
}

/// Force-call-an-MCP-tool tool choice variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeToolChoiceMcp {
    #[serde(rename = "type")]
    pub type_: RealtimeToolChoiceMcpType,
    pub server_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Marker type for `{"type":"mcp"}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeToolChoiceMcpType {
    Mcp,
}

/// `max_output_tokens` value — an integer count or the literal `"inf"`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeMaxOutputTokens {
    /// Explicit integer cap.
    Count(u64),
    /// Literal `"inf"` — unlimited up to model max.
    Inf(InfLiteral),
}

/// Serializes as the literal string `"inf"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InfLiteral {
    Inf,
}

/// Truncation policy for a session — string mode or retention-ratio object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeTruncation {
    /// `"auto"` or `"disabled"`.
    Mode(RealtimeTruncationMode),
    /// `{ type: "retention_ratio", retention_ratio, token_limits? }`.
    RetentionRatio(RealtimeRetentionRatioTruncation),
}

/// Named truncation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeTruncationMode {
    Auto,
    Disabled,
}

/// Retention-ratio truncation config.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimeRetentionRatioTruncation {
    #[serde(rename = "type")]
    pub type_: RealtimeRetentionRatioType,
    pub retention_ratio: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_limits: Option<RealtimeRetentionRatioTokenLimits>,
}

/// Marker type for `{"type":"retention_ratio"}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeRetentionRatioType {
    RetentionRatio,
}

/// Optional custom token limits for retention-ratio truncation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RealtimeRetentionRatioTokenLimits {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_instructions: Option<u64>,
}

/// Value for a single prompt-template variable.
///
/// Per upstream docs, each substitution may be a plain string or one of the
/// Responses API input content parts (`input_text` / `input_image` /
/// `input_file`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimePromptVariableValue {
    /// Plain-string substitution.
    Text(String),
    /// Structured `input_text` part.
    InputText(ResponseInputText),
    /// Structured `input_image` part.
    InputImage(ResponseInputImage),
    /// Structured `input_file` part.
    InputFile(ResponseInputFile),
}

/// Prompt-template reference (`{ id, version?, variables? }`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealtimePromptRef {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Map of variable name to substitution value (string or structured input).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<BTreeMap<String, RealtimePromptVariableValue>>,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Tracing configuration — either the literal `"auto"` or a granular object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeTracing {
    /// `"auto"` — enable tracing with defaults.
    Auto(RealtimeTracingAuto),
    /// Granular tracing configuration object.
    Config(RealtimeTracingConfig),
}

/// Serializes as the literal string `"auto"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeTracingAuto {
    Auto,
}

/// Granular tracing configuration object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeTracingConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Conversation-targeting option on `response.create` (`"auto"` / `"none"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeResponseConversation {
    Auto,
    None,
}

/// Per-response audio override inside [`RealtimeResponseCreateParams`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeResponseCreateAudio {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<RealtimeResponseCreateAudioOutput>,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Output-only audio override allowed on `response.create`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeResponseCreateAudioOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<RealtimeAudioFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<RealtimeVoice>,
    #[serde(flatten, default, skip_serializing_if = "JsonObject::is_empty")]
    pub extra: JsonObject,
}

/// Parameters accepted on the `response.create` event's `response` field.
///
/// This is a session-override subset: any provided field overrides the current
/// session configuration for this response only.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealtimeResponseCreateParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<RealtimeResponseCreateAudio>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation: Option<RealtimeResponseConversation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<Vec<RealtimeConversationItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<RealtimeMaxOutputTokens>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<RealtimeOutputModality>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<RealtimePromptRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<RealtimeToolChoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<RealtimeTool>>,
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
    pub max_output_tokens: Option<RealtimeMaxOutputTokens>,
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
