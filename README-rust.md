# Claude Code SDK for Rust

Rust SDK for Claude Code. This is a port of the Python SDK providing the same functionality in Rust.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
claude-code-sdk = "0.0.10"
tokio = { version = "1.0", features = ["full"] }
tokio-stream = "0.1"
```

**Prerequisites:**
- Rust 1.70+
- Node.js 
- Claude Code: `npm install -g @anthropic-ai/claude-code`

## Quick Start

```rust
use claude_code_sdk::{query, ClaudeCodeOptions};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> claude_code_sdk::Result<()> {
    let mut stream = query("What is 2 + 2?", None).await?;
    
    while let Some(message) = stream.next().await {
        println!("{:?}", message?);
    }
    
    Ok(())
}
```

## Usage

### Basic Query

```rust
use claude_code_sdk::{query, Message, ContentBlock};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> claude_code_sdk::Result<()> {
    let mut stream = query("Hello Claude", None).await?;
    
    while let Some(message) = stream.next().await {
        match message? {
            Message::Assistant(msg) => {
                for block in msg.content {
                    if let ContentBlock::Text(text_block) = block {
                        println!("Assistant: {}", text_block.text);
                    }
                }
            }
            Message::Result(result) => {
                if let Some(exit_code) = result.exit_code {
                    println!("Completed with exit code: {}", exit_code);
                }
            }
            _ => {}
        }
    }
    
    Ok(())
}
```

### With Options

```rust
use claude_code_sdk::{query, ClaudeCodeOptions, PermissionMode};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> claude_code_sdk::Result<()> {
    let options = ClaudeCodeOptions::new()
        .with_system_prompt("You are a helpful assistant")
        .with_max_turns(1)
        .with_permission_mode(PermissionMode::AcceptEdits);

    let mut stream = query("Tell me a joke", Some(options)).await?;
    
    while let Some(message) = stream.next().await {
        // Handle messages...
    }
    
    Ok(())
}
```

### Using Tools

```rust
use claude_code_sdk::{query, ClaudeCodeOptions, PermissionMode};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> claude_code_sdk::Result<()> {
    let options = ClaudeCodeOptions::new()
        .with_allowed_tools(vec!["Read".to_string(), "Write".to_string()])
        .with_permission_mode(PermissionMode::AcceptEdits);

    let mut stream = query(
        "Create a hello.rs file", 
        Some(options)
    ).await?;
    
    while let Some(message) = stream.next().await {
        // Process tool use and results
    }
    
    Ok(())
}
```

### Working Directory

```rust
use claude_code_sdk::{query, ClaudeCodeOptions};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> claude_code_sdk::Result<()> {
    let options = ClaudeCodeOptions::new()
        .with_cwd(PathBuf::from("/path/to/project"));

    let mut stream = query("List files in current directory", Some(options)).await?;
    // Process stream...
    
    Ok(())
}
```

## API Reference

### `query(prompt, options)`

Main async function for querying Claude.

**Parameters:**
- `prompt: &str` - The prompt to send to Claude
- `options: Option<ClaudeCodeOptions>` - Optional configuration

**Returns:** `Result<Pin<Box<dyn Stream<Item = Result<Message>> + Send>>>`

### Types

#### `ClaudeCodeOptions`

Configuration options for Claude Code:

```rust
pub struct ClaudeCodeOptions {
    pub cwd: Option<PathBuf>,
    pub allowed_tools: Option<Vec<String>>,
    pub permission_mode: Option<PermissionMode>,
    pub system_prompt: Option<String>,
    pub max_turns: Option<i32>,
    // ... many other options
}
```

Builder methods:
- `.with_cwd(path)` - Set working directory
- `.with_system_prompt(prompt)` - Set system prompt
- `.with_permission_mode(mode)` - Set permission mode
- `.with_allowed_tools(tools)` - Set allowed tools
- `.with_max_turns(turns)` - Set maximum turns

#### `PermissionMode`

```rust
pub enum PermissionMode {
    Default,        // CLI prompts for dangerous tools
    AcceptEdits,    // Auto-accept file edits
    BypassPermissions, // Allow all tools (use with caution)
}
```

#### Message Types

- `Message::User(UserMessage)` - User messages
- `Message::Assistant(AssistantMessage)` - Assistant responses
- `Message::System(SystemMessage)` - System messages
- `Message::Result(ResultMessage)` - Result/status messages

#### Content Blocks

- `ContentBlock::Text(TextBlock)` - Text content
- `ContentBlock::ToolUse(ToolUseBlock)` - Tool usage
- `ContentBlock::ToolResult(ToolResultBlock)` - Tool results

## Error Handling

```rust
use claude_code_sdk::{
    ClaudeSDKError,    // Base error type
    query,
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    match query("Hello", None).await {
        Ok(mut stream) => {
            while let Some(message) = stream.next().await {
                match message {
                    Ok(msg) => println!("{:?}", msg),
                    Err(e) => match e {
                        ClaudeSDKError::CLINotFound => {
                            eprintln!("Please install Claude Code");
                            break;
                        }
                        ClaudeSDKError::Process { exit_code, stderr } => {
                            eprintln!("Process failed ({}): {}", exit_code, stderr);
                            break;
                        }
                        ClaudeSDKError::CLIJSONDecode { message } => {
                            eprintln!("JSON decode error: {}", message);
                            break;
                        }
                        _ => {
                            eprintln!("Error: {}", e);
                            break;
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Failed to start query: {}", e),
    }
}
```

### Error Types

- `ClaudeSDKError::CLINotFound` - Claude Code not installed
- `ClaudeSDKError::CLIConnection { message }` - Connection issues
- `ClaudeSDKError::Process { exit_code, stderr }` - Process failed
- `ClaudeSDKError::CLIJSONDecode { message }` - JSON parsing issues
- `ClaudeSDKError::Io(std::io::Error)` - I/O errors
- `ClaudeSDKError::Json(serde_json::Error)` - JSON errors

## Available Tools

See the [Claude Code documentation](https://docs.anthropic.com/en/docs/claude-code/security#tools-available-to-claude) for a complete list of available tools.

## Examples

Run the quick start example:

```bash
cargo run --example quick_start
```

## Differences from Python SDK

This Rust SDK maintains API compatibility with the Python SDK while providing:

1. **Type Safety**: Full compile-time type checking
2. **Performance**: Lower memory usage and faster execution  
3. **Async Streams**: Uses Rust's `Stream` trait instead of Python's `AsyncIterator`
4. **Builder Pattern**: Fluent API for options configuration
5. **Comprehensive Error Types**: Rich error handling with `thiserror`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run `cargo test` and `cargo clippy`
6. Submit a pull request

## License

MIT