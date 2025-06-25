use claude_code_sdk::types::*;
use serde_json;

#[test]
fn test_permission_mode_serialization() {
    let default_mode = PermissionMode::Default;
    let accept_edits = PermissionMode::AcceptEdits;
    let bypass = PermissionMode::BypassPermissions;

    assert_eq!(serde_json::to_string(&default_mode).unwrap(), "\"default\"");
    assert_eq!(
        serde_json::to_string(&accept_edits).unwrap(),
        "\"accept_edits\""
    );
    assert_eq!(
        serde_json::to_string(&bypass).unwrap(),
        "\"bypass_permissions\""
    );
}

#[test]
fn test_permission_mode_deserialization() {
    let default_mode: PermissionMode = serde_json::from_str("\"default\"").unwrap();
    let accept_edits: PermissionMode = serde_json::from_str("\"accept_edits\"").unwrap();
    let bypass: PermissionMode = serde_json::from_str("\"bypass_permissions\"").unwrap();

    assert_eq!(default_mode, PermissionMode::Default);
    assert_eq!(accept_edits, PermissionMode::AcceptEdits);
    assert_eq!(bypass, PermissionMode::BypassPermissions);
}

#[test]
fn test_text_block_creation() {
    let block = TextBlock::new("Hello, world!");

    assert_eq!(block.block_type, "text");
    assert_eq!(block.text, "Hello, world!");
}

#[test]
fn test_text_block_serialization() {
    let block = TextBlock::new("Test text");
    let json = serde_json::to_string(&block).unwrap();

    assert!(json.contains("\"type\":\"text\""));
    assert!(json.contains("\"text\":\"Test text\""));
}

#[test]
fn test_tool_use_block_creation() {
    let input = serde_json::json!({"param": "value"});
    let block = ToolUseBlock::new("tool-123", "read_file", input.clone());

    assert_eq!(block.block_type, "tool_use");
    assert_eq!(block.id, "tool-123");
    assert_eq!(block.name, "read_file");
    assert_eq!(block.input, input);
}

#[test]
fn test_tool_result_block_creation() {
    let block = ToolResultBlock::new("tool-123", Some("File contents"), Some(false));

    assert_eq!(block.block_type, "tool_result");
    assert_eq!(block.tool_use_id, "tool-123");
    assert_eq!(block.content, Some("File contents".to_string()));
    assert_eq!(block.is_error, Some(false));
}

#[test]
fn test_user_message_creation() {
    let text_block = TextBlock::new("Hello");
    let content = vec![ContentBlock::Text(text_block)];
    let message = UserMessage::new(content);

    assert_eq!(message.message_type, "user");
    assert_eq!(message.content.len(), 1);
}

#[test]
fn test_assistant_message_creation() {
    let text_block = TextBlock::new("Hi there!");
    let content = vec![ContentBlock::Text(text_block)];
    let message = AssistantMessage::new(content);

    assert_eq!(message.message_type, "assistant");
    assert_eq!(message.content.len(), 1);
}

#[test]
fn test_system_message_creation() {
    let message = SystemMessage::new("You are a helpful assistant");

    assert_eq!(message.message_type, "system");
    assert_eq!(message.content, "You are a helpful assistant");
}

#[test]
fn test_result_message_creation() {
    let message = ResultMessage::new("result-123");

    assert_eq!(message.message_type, "result");
    assert_eq!(message.id, "result-123");
    assert_eq!(message.exit_code, None);
    assert_eq!(message.content, None);
}

#[test]
fn test_claude_code_options_default() {
    let options = ClaudeCodeOptions::default();

    assert!(options.cwd.is_none());
    assert!(options.allowed_tools.is_none());
    assert!(options.permission_mode.is_none());
    assert!(options.system_prompt.is_none());
    assert!(options.max_turns.is_none());
}

#[test]
fn test_claude_code_options_builder() {
    let options = ClaudeCodeOptions::new()
        .with_system_prompt("Test prompt")
        .with_max_turns(5)
        .with_permission_mode(PermissionMode::AcceptEdits)
        .with_allowed_tools(vec!["Read".to_string(), "Write".to_string()]);

    assert_eq!(options.system_prompt, Some("Test prompt".to_string()));
    assert_eq!(options.max_turns, Some(5));
    assert_eq!(options.permission_mode, Some(PermissionMode::AcceptEdits));
    assert_eq!(
        options.allowed_tools,
        Some(vec!["Read".to_string(), "Write".to_string()])
    );
}

#[test]
fn test_content_block_from_text() {
    let text_block = TextBlock::new("Test");
    let content_block: ContentBlock = text_block.into();

    match content_block {
        ContentBlock::Text(block) => {
            assert_eq!(block.text, "Test");
        }
        _ => panic!("Expected Text variant"),
    }
}

#[test]
fn test_message_from_user() {
    let text_block = TextBlock::new("Hello");
    let content = vec![ContentBlock::Text(text_block)];
    let user_message = UserMessage::new(content);
    let message: Message = user_message.into();

    match message {
        Message::User(msg) => {
            assert_eq!(msg.message_type, "user");
        }
        _ => panic!("Expected User variant"),
    }
}
