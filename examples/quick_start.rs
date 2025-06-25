use claude_code_sdk::{query, ClaudeCodeOptions, ContentBlock, Message, PermissionMode};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> claude_code_sdk::Result<()> {
    println!("Claude Code SDK for Rust - Quick Start Examples");
    println!("================================================");

    // Example 1: Simple query
    println!("\n1. Simple query:");
    simple_query().await?;

    // Example 2: Query with options
    println!("\n2. Query with system prompt and options:");
    query_with_options().await?;

    // Example 3: Query with tools enabled
    println!("\n3. Query with tools enabled:");
    query_with_tools().await?;

    // Example 4: Handling different message types
    println!("\n4. Handling different message types:");
    handle_message_types().await?;

    Ok(())
}

async fn simple_query() -> claude_code_sdk::Result<()> {
    let mut stream = query("What is 2 + 2?", None).await?;

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
                    println!("Query completed with exit code: {}", exit_code);
                }
                if let Some(tokens_input) = result.tokens_input {
                    println!("Input tokens: {}", tokens_input);
                }
                if let Some(tokens_output) = result.tokens_output {
                    println!("Output tokens: {}", tokens_output);
                }
            }
            _ => {}
        }
    }

    Ok(())
}

async fn query_with_options() -> claude_code_sdk::Result<()> {
    let options = ClaudeCodeOptions::new()
        .with_system_prompt("You are a helpful math tutor. Always show your work.")
        .with_max_turns(1);

    let mut stream = query("Solve this equation: 3x + 5 = 14", Some(options)).await?;

    while let Some(message) = stream.next().await {
        match message? {
            Message::Assistant(msg) => {
                for block in msg.content {
                    if let ContentBlock::Text(text_block) = block {
                        println!("Math Tutor: {}", text_block.text);
                    }
                }
            }
            Message::Result(result) => {
                println!("Query completed.");
                if let Some(cost) = result.cost_usd {
                    println!("Cost: ${:.6}", cost);
                }
            }
            _ => {}
        }
    }

    Ok(())
}

async fn query_with_tools() -> claude_code_sdk::Result<()> {
    let options = ClaudeCodeOptions::new()
        .with_allowed_tools(vec!["Read".to_string(), "Write".to_string()])
        .with_permission_mode(PermissionMode::AcceptEdits)
        .with_system_prompt("You are a helpful programming assistant.");

    let mut stream = query(
        "Create a simple hello.rs file with a main function that prints 'Hello, Rust!'",
        Some(options),
    )
    .await?;

    while let Some(message) = stream.next().await {
        match message? {
            Message::Assistant(msg) => {
                for block in msg.content {
                    match block {
                        ContentBlock::Text(text_block) => {
                            println!("Assistant: {}", text_block.text);
                        }
                        ContentBlock::ToolUse(tool_block) => {
                            println!("Using tool: {} (ID: {})", tool_block.name, tool_block.id);
                            println!("Input: {}", tool_block.input);
                        }
                        _ => {}
                    }
                }
            }
            Message::Result(result) => {
                println!(
                    "Tool operation completed with exit code: {:?}",
                    result.exit_code
                );
            }
            _ => {}
        }
    }

    Ok(())
}

async fn handle_message_types() -> claude_code_sdk::Result<()> {
    let mut stream = query("Tell me a joke", None).await?;

    while let Some(message) = stream.next().await {
        match message? {
            Message::User(msg) => {
                println!("User message with {} content blocks", msg.content.len());
            }
            Message::Assistant(msg) => {
                println!("Assistant message:");
                for (i, block) in msg.content.iter().enumerate() {
                    match block {
                        ContentBlock::Text(text_block) => {
                            println!("  Block {}: Text - {}", i, text_block.text);
                        }
                        ContentBlock::ToolUse(tool_block) => {
                            println!(
                                "  Block {}: Tool Use - {} (ID: {})",
                                i, tool_block.name, tool_block.id
                            );
                        }
                        ContentBlock::ToolResult(result_block) => {
                            println!(
                                "  Block {}: Tool Result - ID: {}",
                                i, result_block.tool_use_id
                            );
                            if let Some(content) = &result_block.content {
                                println!("    Content: {}", content);
                            }
                            if let Some(is_error) = result_block.is_error {
                                println!("    Is Error: {}", is_error);
                            }
                        }
                    }
                }
            }
            Message::System(msg) => {
                println!("System message: {}", msg.content);
            }
            Message::Result(result) => {
                println!("Result message (ID: {})", result.id);
                if let Some(exit_code) = result.exit_code {
                    println!("  Exit code: {}", exit_code);
                }
                if let Some(content) = &result.content {
                    println!("  Content: {}", content);
                }
                if let Some(cost) = result.cost_usd {
                    println!("  Cost: ${:.6}", cost);
                }
                if let Some(tokens_in) = result.tokens_input {
                    println!("  Input tokens: {}", tokens_in);
                }
                if let Some(tokens_out) = result.tokens_output {
                    println!("  Output tokens: {}", tokens_out);
                }
                if let Some(reasoning_tokens) = result.reasoning_tokens {
                    println!("  Reasoning tokens: {}", reasoning_tokens);
                }
                if let Some(canceled) = result.canceled {
                    println!("  Canceled: {}", canceled);
                }
            }
        }
    }

    Ok(())
}

// Helper function to demonstrate error handling
#[allow(dead_code)]
async fn error_handling_example() {
    use claude_code_sdk::ClaudeSDKError;

    let result = query("Test query", None).await;

    match result {
        Ok(mut stream) => {
            while let Some(message) = stream.next().await {
                match message {
                    Ok(msg) => {
                        println!("Received message: {:?}", msg);
                    }
                    Err(e) => match e {
                        ClaudeSDKError::CLINotFound => {
                            eprintln!("Error: Claude Code CLI not found. Please install it.");
                            break;
                        }
                        ClaudeSDKError::CLIConnection { message } => {
                            eprintln!("Connection error: {}", message);
                            break;
                        }
                        ClaudeSDKError::Process { exit_code, stderr } => {
                            eprintln!("Process failed (exit code {}): {}", exit_code, stderr);
                            break;
                        }
                        ClaudeSDKError::CLIJSONDecode { message } => {
                            eprintln!("JSON decode error: {}", message);
                            break;
                        }
                        _ => {
                            eprintln!("Other error: {}", e);
                            break;
                        }
                    },
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to start query: {}", e);
        }
    }
}
