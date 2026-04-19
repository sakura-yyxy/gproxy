//! Client-to-server events over the OpenAI Realtime WebSocket.
//!
//! Modeled from `upstream_docs/openai/docs/Realtime client events.md`. The
//! union discriminates on the `type` field.

use serde::{Deserialize, Serialize};

use super::types::{JsonObject, RealtimeConversationItem, RealtimeSession};

/// `session.update` — update the Realtime session configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeSessionUpdate {
    pub session: RealtimeSession,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// `input_audio_buffer.append` — append base64 audio bytes to the input buffer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeInputAudioBufferAppend {
    pub audio: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// `input_audio_buffer.commit` — commit the current input audio buffer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OpenAiRealtimeInputAudioBufferCommit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// `input_audio_buffer.clear` — discard the current input audio buffer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OpenAiRealtimeInputAudioBufferClear {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// `conversation.item.create` — add an item to the conversation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeConversationItemCreate {
    pub item: RealtimeConversationItem,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
}

/// `conversation.item.retrieve` — fetch the server view of a conversation item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenAiRealtimeConversationItemRetrieve {
    pub item_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// `conversation.item.truncate` — truncate an assistant audio message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRealtimeConversationItemTruncate {
    pub audio_end_ms: i64,
    pub content_index: i64,
    pub item_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// `conversation.item.delete` — remove an item from the conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenAiRealtimeConversationItemDelete {
    pub item_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// `response.create` — request model inference / response generation.
///
/// The `response` field mirrors `RealtimeResponseCreateParams` which is a
/// large, overlapping subset of the session configuration. We retain it as a
/// [`JsonObject`] so the wire representation is preserved verbatim without
/// re-modeling the entire session schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenAiRealtimeResponseCreate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<JsonObject>,
}

/// `response.cancel` — cancel an in-progress response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OpenAiRealtimeResponseCancel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_id: Option<String>,
}

/// `output_audio_buffer.clear` — WebRTC/SIP only: cut off the active audio
/// response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OpenAiRealtimeOutputAudioBufferClear {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// Union of every client-to-server Realtime event (11 variants).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(clippy::large_enum_variant)]
pub enum OpenAiRealtimeClientEvent {
    #[serde(rename = "session.update")]
    SessionUpdate(OpenAiRealtimeSessionUpdate),
    #[serde(rename = "input_audio_buffer.append")]
    InputAudioBufferAppend(OpenAiRealtimeInputAudioBufferAppend),
    #[serde(rename = "input_audio_buffer.commit")]
    InputAudioBufferCommit(OpenAiRealtimeInputAudioBufferCommit),
    #[serde(rename = "input_audio_buffer.clear")]
    InputAudioBufferClear(OpenAiRealtimeInputAudioBufferClear),
    #[serde(rename = "conversation.item.create")]
    ConversationItemCreate(OpenAiRealtimeConversationItemCreate),
    #[serde(rename = "conversation.item.retrieve")]
    ConversationItemRetrieve(OpenAiRealtimeConversationItemRetrieve),
    #[serde(rename = "conversation.item.truncate")]
    ConversationItemTruncate(OpenAiRealtimeConversationItemTruncate),
    #[serde(rename = "conversation.item.delete")]
    ConversationItemDelete(OpenAiRealtimeConversationItemDelete),
    #[serde(rename = "response.create")]
    ResponseCreate(OpenAiRealtimeResponseCreate),
    #[serde(rename = "response.cancel")]
    ResponseCancel(OpenAiRealtimeResponseCancel),
    #[serde(rename = "output_audio_buffer.clear")]
    OutputAudioBufferClear(OpenAiRealtimeOutputAudioBufferClear),
}
