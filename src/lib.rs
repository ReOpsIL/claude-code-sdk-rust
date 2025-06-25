//! Claude Code SDK for Rust
//!
//! This crate provides a Rust SDK for interacting with Claude Code.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use claude_code_sdk::{query, ClaudeCodeOptions};
//! use tokio_stream::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> claude_code_sdk::Result<()> {
//!     let mut stream = query("What is 2 + 2?", None).await?;
//!     
//!     while let Some(message) = stream.next().await {
//!         println!("{:?}", message?);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! ```rust,no_run
//! use claude_code_sdk::{query, ClaudeCodeOptions, PermissionMode};
//! use tokio_stream::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> claude_code_sdk::Result<()> {
//!     let options = ClaudeCodeOptions::new()
//!         .with_system_prompt("You are a helpful assistant")
//!         .with_permission_mode(PermissionMode::AcceptEdits)
//!         .with_max_turns(1);
//!
//!     let mut stream = query("Create a hello.py file", Some(options)).await?;
//!     
//!     while let Some(message) = stream.next().await {
//!         println!("{:?}", message?);
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod transport;
pub mod types;

use client::InternalClient;
pub use error::{ClaudeSDKError, Result};
use futures::stream::Stream;
use std::env;
use std::pin::Pin;
pub use types::*;

/// Query Claude Code with a prompt and optional configuration.
///
/// This is the main entry point for the SDK. It creates a client, connects to
/// the Claude Code CLI, and returns a stream of messages from the conversation.
///
/// # Arguments
///
/// * `prompt` - The prompt to send to Claude
/// * `options` - Optional configuration (uses defaults if None)
///
/// # Returns
///
/// An async stream of `Message` objects representing the conversation
///
/// # Example
///
/// ```rust,no_run
/// use claude_code_sdk::{query, ClaudeCodeOptions};
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> claude_code_sdk::Result<()> {
///     let mut stream = query("Hello Claude", None).await?;
///     
///     while let Some(message) = stream.next().await {
///         match message? {
///             claude_code_sdk::Message::Assistant(msg) => {
///                 for block in msg.content {
///                     if let claude_code_sdk::ContentBlock::Text(text_block) = block {
///                         println!("{}", text_block.text);
///                     }
///                 }
///             }
///             _ => {}
///         }
///     }
///     
///     Ok(())
/// }
/// ```
pub async fn query(
    prompt: &str,
    options: Option<ClaudeCodeOptions>,
) -> Result<Pin<Box<dyn Stream<Item = Result<Message>> + Send>>> {
    // Set environment variable to identify SDK usage
    env::set_var("CLAUDE_CODE_ENTRYPOINT", "sdk-rust");

    let options = options.unwrap_or_default();
    let mut client = InternalClient::new();

    client.process_query(prompt.to_string(), options).await
}

// Re-export commonly used types at the crate root
pub use error::ClaudeSDKError as Error;
pub use transport::Transport;
