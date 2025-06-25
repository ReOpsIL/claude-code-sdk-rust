# Claude Code SDK for Rust

Rust SDK for Claude Code. See the [Claude Code SDK documentation](https://docs.anthropic.com/en/docs/claude-code/sdk) for more information.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
claude-code-sdk = "0.1.0"
```

**Prerequisites:**
- Rust 1.70+
- Node.js 
- Claude Code: `npm install -g @anthropic-ai/claude-code`

## Quick Start

```rust
use claude_code_sdk::query;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = query("What is 2 + 2?", None).await?;
    
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => println!("{:?}", msg),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}
```

## Usage

### Basic Query

```rust
use claude_code_sdk::{query, ClaudeCodeOptions, Message};

// Simple query
let mut stream = query("Hello Claude", None).await?;
while let Some(message) = stream.next().await {
    if let Ok(Message::Assistant(msg)) = message {
        for block in &msg.content {
            if let ContentBlock::Text(text_block) = block {
                println!("{}", text_block.text);
            }
        }
    }
}

// With options
let options = ClaudeCodeOptions {
    system_prompt: Some("You are a helpful assistant".to_string()),
    max_turns: Some(1),
    ..Default::default()
};

let mut stream = query("Tell me a joke", Some(options)).await?;
```

### Using Tools

```rust
let options = ClaudeCodeOptions {
    allowed_tools: Some(vec!["Read".to_string(), "Write".to_string(), "Bash".to_string()]),
    permission_mode: Some("acceptEdits".to_string()),
    ..Default::default()
};

let mut stream = query("Create a hello.rs file", Some(options)).await?;
```

### Working Directory

```rust
use std::path::PathBuf;

let options = ClaudeCodeOptions {
    cwd: Some(PathBuf::from("/path/to/project")),
    ..Default::default()
};
```

## API Reference

### `query(prompt, options)`

Main async function for querying Claude.

**Parameters:**
- `prompt: &str` - The prompt to send to Claude
- `options: Option<ClaudeCodeOptions>` - Optional configuration

**Returns:** `Result<MessageStream, ClaudeSDKError>` - Stream of response messages

### Types

See the library documentation for complete type definitions:
- `ClaudeCodeOptions` - Configuration options
- `Message` - Enum for different message types
- `ContentBlock` - Enum for different content block types

## Error Handling

```rust
use claude_code_sdk::{
    ClaudeSDKError,
    query,
};

match query("Hello", None).await {
    Ok(mut stream) => {
        while let Some(message) = stream.next().await {
            match message {
                Ok(msg) => println!("{:?}", msg),
                Err(e) => eprintln!("Stream error: {}", e),
            }
        }
    }
    Err(ClaudeSDKError::CLINotFound) => {
        eprintln!("Please install Claude Code");
    }
    Err(ClaudeSDKError::ProcessError { exit_code, .. }) => {
        eprintln!("Process failed with exit code: {}", exit_code);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Available Tools

See the [Claude Code documentation](https://docs.anthropic.com/en/docs/claude-code/security#tools-available-to-claude) for a complete list of available tools.

## Examples

See the `examples/` directory for complete working examples.

## License

MIT