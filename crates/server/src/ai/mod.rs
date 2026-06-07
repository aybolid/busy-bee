#![allow(dead_code)]

mod chat;
mod client;

#[allow(unused_imports)]
pub use chat::{ChatMessage, ChatRequest, ChatResponse, ChatRole, ExecChatError, Message, Usage};
pub use client::{ApiKey, Client, ModelName, create_ai_client};
