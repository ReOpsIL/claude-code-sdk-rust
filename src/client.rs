use crate::error::Result;
use crate::transport::{SubprocessCLITransport, Transport};
use crate::types::{ClaudeCodeOptions, Message};
use futures::stream::Stream;
use std::pin::Pin;

pub struct InternalClient {
    transport: Option<Box<dyn Transport>>,
}

impl InternalClient {
    pub fn new() -> Self {
        Self { transport: None }
    }

    pub async fn process_query(
        &mut self,
        prompt: String,
        options: ClaudeCodeOptions,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Message>> + Send>>> {
        // Create and configure transport
        let mut transport = Box::new(SubprocessCLITransport::new(prompt, options));

        // Connect to the transport
        transport.connect().await?;

        // Get the message stream
        let message_stream = transport.receive_messages().await?;

        // Store the transport for cleanup
        self.transport = Some(transport);

        // Return the stream of messages
        Ok(message_stream)
    }
}

impl Drop for InternalClient {
    fn drop(&mut self) {
        if let Some(_transport) = self.transport.take() {
            // Best effort cleanup - we can't await in Drop
            // The transport's Drop implementation should handle cleanup
        }
    }
}
