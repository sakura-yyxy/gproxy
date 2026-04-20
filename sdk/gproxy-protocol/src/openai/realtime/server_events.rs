//! Server-to-client events over the OpenAI Realtime WebSocket.
//!
//! Modeled from `upstream_docs/openai/docs/Realtime server events.md`. The
//! union discriminates on the `type` field.

use serde::{Deserialize, Serialize};

use super::types::{
    RealtimeConversationItem, RealtimeConversationResource, RealtimeError, RealtimeLogProb,
    RealtimeRateLimit, RealtimeResponse, RealtimeSession, RealtimeStreamingContentPart,
    RealtimeTranscriptionError, RealtimeTranscriptionUsage,
};

/// `error` — server-reported error event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeErrorEvent {
    pub error: RealtimeError,
    pub event_id: String,
}

/// `session.created` — first event after connection, delivering default config.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeSessionCreated {
    pub event_id: String,
    pub session: RealtimeSession,
}

/// `session.updated` — emitted after a `session.update` client event applies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeSessionUpdated {
    pub event_id: String,
    pub session: RealtimeSession,
}

/// `conversation.item.added` — an item was appended to the conversation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeConversationItemAdded {
    pub event_id: String,
    pub item: RealtimeConversationItem,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
}

/// `conversation.item.done` — an item has been finalized.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeConversationItemDone {
    pub event_id: String,
    pub item: RealtimeConversationItem,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
}

/// `conversation.item.input_audio_transcription.completed` — ASR finished for
/// a user audio item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeInputAudioTranscriptionCompleted {
    pub content_index: i64,
    pub event_id: String,
    pub item_id: String,
    pub transcript: String,
    pub usage: RealtimeTranscriptionUsage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<RealtimeLogProb>>,
}

/// `conversation.item.input_audio_transcription.delta` — streaming ASR delta.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeInputAudioTranscriptionDelta {
    pub event_id: String,
    pub item_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_index: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<RealtimeLogProb>>,
    /// Present when the delta text has been obfuscated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obfuscation: Option<String>,
}

/// `conversation.item.input_audio_transcription.segment` — diarized segment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeInputAudioTranscriptionSegment {
    pub id: String,
    pub content_index: i64,
    pub end: f64,
    pub event_id: String,
    pub item_id: String,
    pub speaker: String,
    pub start: f64,
    pub text: String,
}

/// `conversation.item.input_audio_transcription.failed` — ASR failed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeInputAudioTranscriptionFailed {
    pub content_index: i64,
    pub error: RealtimeTranscriptionError,
    pub event_id: String,
    pub item_id: String,
}

/// `conversation.item.truncated` — an assistant audio item was truncated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeConversationItemTruncated {
    pub audio_end_ms: i64,
    pub content_index: i64,
    pub event_id: String,
    pub item_id: String,
}

/// `conversation.item.deleted` — an item has been removed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeConversationItemDeleted {
    pub event_id: String,
    pub item_id: String,
}

/// `input_audio_buffer.committed` — a buffer commit has produced a user item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeInputAudioBufferCommitted {
    pub event_id: String,
    pub item_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
}

/// `input_audio_buffer.dtmf_event_received` — SIP only DTMF keypress.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeInputAudioBufferDtmfEventReceived {
    pub event: String,
    pub received_at: f64,
}

/// `input_audio_buffer.cleared` — buffer was cleared by the client.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeInputAudioBufferCleared {
    pub event_id: String,
}

/// `input_audio_buffer.speech_started` — server VAD detected speech start.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeInputAudioBufferSpeechStarted {
    pub audio_start_ms: i64,
    pub event_id: String,
    pub item_id: String,
}

/// `input_audio_buffer.speech_stopped` — server VAD detected end of speech.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeInputAudioBufferSpeechStopped {
    pub audio_end_ms: i64,
    pub event_id: String,
    pub item_id: String,
}

/// `input_audio_buffer.timeout_triggered` — idle VAD timeout fired.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeInputAudioBufferTimeoutTriggered {
    pub audio_end_ms: i64,
    pub audio_start_ms: i64,
    pub event_id: String,
    pub item_id: String,
}

/// `response.created` — a new response has entered `in_progress` state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseCreated {
    pub event_id: String,
    pub response: RealtimeResponse,
}

/// `response.done` — a response finished streaming (any terminal status).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseDone {
    pub event_id: String,
    pub response: RealtimeResponse,
}

/// `response.output_item.added` — a new output item is being generated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseOutputItemAdded {
    pub event_id: String,
    pub item: RealtimeConversationItem,
    pub output_index: i64,
    pub response_id: String,
}

/// `response.output_item.done` — an output item finished streaming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseOutputItemDone {
    pub event_id: String,
    pub item: RealtimeConversationItem,
    pub output_index: i64,
    pub response_id: String,
}

/// `response.content_part.added` — a new content part was added.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseContentPartAdded {
    pub content_index: i64,
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
    pub part: RealtimeStreamingContentPart,
    pub response_id: String,
}

/// `response.content_part.done` — a content part finished streaming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseContentPartDone {
    pub content_index: i64,
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
    pub part: RealtimeStreamingContentPart,
    pub response_id: String,
}

/// `response.output_text.delta` — streaming text content part delta.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseOutputTextDelta {
    pub content_index: i64,
    pub delta: String,
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
    pub response_id: String,
}

/// `response.output_text.done` — final text for a text content part.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseOutputTextDone {
    pub content_index: i64,
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
    pub response_id: String,
    pub text: String,
}

/// `response.output_audio_transcript.delta` — streaming audio transcript delta.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseOutputAudioTranscriptDelta {
    pub content_index: i64,
    pub delta: String,
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
    pub response_id: String,
}

/// `response.output_audio_transcript.done` — final audio transcript.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseOutputAudioTranscriptDone {
    pub content_index: i64,
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
    pub response_id: String,
    pub transcript: String,
}

/// `response.output_audio.delta` — base64 audio bytes delta.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseOutputAudioDelta {
    pub content_index: i64,
    pub delta: String,
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
    pub response_id: String,
}

/// `response.output_audio.done` — audio content part is complete.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseOutputAudioDone {
    pub content_index: i64,
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
    pub response_id: String,
}

/// `response.function_call_arguments.delta` — streaming function arguments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseFunctionCallArgumentsDelta {
    pub call_id: String,
    pub delta: String,
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
    pub response_id: String,
}

/// `response.function_call_arguments.done` — final function arguments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseFunctionCallArgumentsDone {
    pub arguments: String,
    pub call_id: String,
    pub event_id: String,
    pub item_id: String,
    pub name: String,
    pub output_index: i64,
    pub response_id: String,
}

/// `response.mcp_call_arguments.delta` — streaming MCP call arguments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseMcpCallArgumentsDelta {
    pub delta: String,
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
    pub response_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obfuscation: Option<String>,
}

/// `response.mcp_call_arguments.done` — final MCP call arguments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseMcpCallArgumentsDone {
    pub arguments: String,
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
    pub response_id: String,
}

/// `response.mcp_call.in_progress` — MCP tool invocation started.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseMcpCallInProgress {
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
}

/// `response.mcp_call.completed` — MCP tool invocation succeeded.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseMcpCallCompleted {
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
}

/// `response.mcp_call.failed` — MCP tool invocation failed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeResponseMcpCallFailed {
    pub event_id: String,
    pub item_id: String,
    pub output_index: i64,
}

/// `mcp_list_tools.in_progress` — MCP list-tools started.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeMcpListToolsInProgress {
    pub event_id: String,
    pub item_id: String,
}

/// `mcp_list_tools.completed` — MCP list-tools completed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeMcpListToolsCompleted {
    pub event_id: String,
    pub item_id: String,
}

/// `mcp_list_tools.failed` — MCP list-tools failed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeMcpListToolsFailed {
    pub event_id: String,
    pub item_id: String,
}

/// `rate_limits.updated` — updated rate-limit info for the session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeRateLimitsUpdated {
    pub event_id: String,
    pub rate_limits: Vec<RealtimeRateLimit>,
}

/// `conversation.item.created` — a conversation item was created server-side.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeConversationItemCreated {
    pub event_id: String,
    pub item: RealtimeConversationItem,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
}

/// `conversation.created` — emitted right after session creation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeConversationCreated {
    pub conversation: RealtimeConversationResource,
    pub event_id: String,
}

/// Union of every server-to-client Realtime event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OpenAiRealtimeServerEvent {
    #[serde(rename = "error")]
    Error(OpenAiRealtimeErrorEvent),
    #[serde(rename = "session.created")]
    SessionCreated(OpenAiRealtimeSessionCreated),
    #[serde(rename = "session.updated")]
    SessionUpdated(OpenAiRealtimeSessionUpdated),
    #[serde(rename = "conversation.item.added")]
    ConversationItemAdded(OpenAiRealtimeConversationItemAdded),
    #[serde(rename = "conversation.item.done")]
    ConversationItemDone(OpenAiRealtimeConversationItemDone),
    #[serde(rename = "conversation.item.input_audio_transcription.completed")]
    ConversationItemInputAudioTranscriptionCompleted(
        OpenAiRealtimeInputAudioTranscriptionCompleted,
    ),
    #[serde(rename = "conversation.item.input_audio_transcription.delta")]
    ConversationItemInputAudioTranscriptionDelta(OpenAiRealtimeInputAudioTranscriptionDelta),
    #[serde(rename = "conversation.item.input_audio_transcription.segment")]
    ConversationItemInputAudioTranscriptionSegment(OpenAiRealtimeInputAudioTranscriptionSegment),
    #[serde(rename = "conversation.item.input_audio_transcription.failed")]
    ConversationItemInputAudioTranscriptionFailed(OpenAiRealtimeInputAudioTranscriptionFailed),
    #[serde(rename = "conversation.item.truncated")]
    ConversationItemTruncated(OpenAiRealtimeConversationItemTruncated),
    #[serde(rename = "conversation.item.deleted")]
    ConversationItemDeleted(OpenAiRealtimeConversationItemDeleted),
    #[serde(rename = "input_audio_buffer.committed")]
    InputAudioBufferCommitted(OpenAiRealtimeInputAudioBufferCommitted),
    #[serde(rename = "input_audio_buffer.dtmf_event_received")]
    InputAudioBufferDtmfEventReceived(OpenAiRealtimeInputAudioBufferDtmfEventReceived),
    #[serde(rename = "input_audio_buffer.cleared")]
    InputAudioBufferCleared(OpenAiRealtimeInputAudioBufferCleared),
    #[serde(rename = "input_audio_buffer.speech_started")]
    InputAudioBufferSpeechStarted(OpenAiRealtimeInputAudioBufferSpeechStarted),
    #[serde(rename = "input_audio_buffer.speech_stopped")]
    InputAudioBufferSpeechStopped(OpenAiRealtimeInputAudioBufferSpeechStopped),
    #[serde(rename = "input_audio_buffer.timeout_triggered")]
    InputAudioBufferTimeoutTriggered(OpenAiRealtimeInputAudioBufferTimeoutTriggered),
    #[serde(rename = "response.created")]
    ResponseCreated(OpenAiRealtimeResponseCreated),
    #[serde(rename = "response.done")]
    ResponseDone(OpenAiRealtimeResponseDone),
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded(OpenAiRealtimeResponseOutputItemAdded),
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone(OpenAiRealtimeResponseOutputItemDone),
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded(OpenAiRealtimeResponseContentPartAdded),
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone(OpenAiRealtimeResponseContentPartDone),
    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta(OpenAiRealtimeResponseOutputTextDelta),
    #[serde(rename = "response.output_text.done")]
    ResponseOutputTextDone(OpenAiRealtimeResponseOutputTextDone),
    #[serde(rename = "response.output_audio_transcript.delta")]
    ResponseOutputAudioTranscriptDelta(OpenAiRealtimeResponseOutputAudioTranscriptDelta),
    #[serde(rename = "response.output_audio_transcript.done")]
    ResponseOutputAudioTranscriptDone(OpenAiRealtimeResponseOutputAudioTranscriptDone),
    #[serde(rename = "response.output_audio.delta")]
    ResponseOutputAudioDelta(OpenAiRealtimeResponseOutputAudioDelta),
    #[serde(rename = "response.output_audio.done")]
    ResponseOutputAudioDone(OpenAiRealtimeResponseOutputAudioDone),
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta(OpenAiRealtimeResponseFunctionCallArgumentsDelta),
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone(OpenAiRealtimeResponseFunctionCallArgumentsDone),
    #[serde(rename = "response.mcp_call_arguments.delta")]
    ResponseMcpCallArgumentsDelta(OpenAiRealtimeResponseMcpCallArgumentsDelta),
    #[serde(rename = "response.mcp_call_arguments.done")]
    ResponseMcpCallArgumentsDone(OpenAiRealtimeResponseMcpCallArgumentsDone),
    #[serde(rename = "response.mcp_call.in_progress")]
    ResponseMcpCallInProgress(OpenAiRealtimeResponseMcpCallInProgress),
    #[serde(rename = "response.mcp_call.completed")]
    ResponseMcpCallCompleted(OpenAiRealtimeResponseMcpCallCompleted),
    #[serde(rename = "response.mcp_call.failed")]
    ResponseMcpCallFailed(OpenAiRealtimeResponseMcpCallFailed),
    #[serde(rename = "mcp_list_tools.in_progress")]
    McpListToolsInProgress(OpenAiRealtimeMcpListToolsInProgress),
    #[serde(rename = "mcp_list_tools.completed")]
    McpListToolsCompleted(OpenAiRealtimeMcpListToolsCompleted),
    #[serde(rename = "mcp_list_tools.failed")]
    McpListToolsFailed(OpenAiRealtimeMcpListToolsFailed),
    #[serde(rename = "rate_limits.updated")]
    RateLimitsUpdated(OpenAiRealtimeRateLimitsUpdated),
    #[serde(rename = "conversation.item.created")]
    ConversationItemCreated(OpenAiRealtimeConversationItemCreated),
    #[serde(rename = "conversation.created")]
    ConversationCreated(OpenAiRealtimeConversationCreated),
}
