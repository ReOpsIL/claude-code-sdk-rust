use claude_code_sdk::error::*;
use std::io;

#[test]
fn test_cli_connection_error() {
    let error = ClaudeSDKError::cli_connection("Connection failed");

    match &error {
        ClaudeSDKError::CLIConnection { message } => {
            assert_eq!(message, "Connection failed");
        }
        _ => panic!("Expected CLIConnection variant"),
    }

    assert_eq!(error.to_string(), "CLI connection error: Connection failed");
}

#[test]
fn test_cli_not_found_error() {
    let error = ClaudeSDKError::CLINotFound;

    assert_eq!(
        error.to_string(),
        "Claude Code CLI not found. Please install it with: npm install -g @anthropic-ai/claude-code"
    );
}

#[test]
fn test_process_error() {
    let error = ClaudeSDKError::process(1, "Command failed");

    match &error {
        ClaudeSDKError::Process { exit_code, stderr } => {
            assert_eq!(*exit_code, 1);
            assert_eq!(stderr, "Command failed");
        }
        _ => panic!("Expected Process variant"),
    }

    assert_eq!(
        error.to_string(),
        "Process failed with exit code 1: Command failed"
    );
}

#[test]
fn test_cli_json_decode_error() {
    let error = ClaudeSDKError::cli_json_decode("Invalid JSON");

    match &error {
        ClaudeSDKError::CLIJSONDecode { message } => {
            assert_eq!(message, "Invalid JSON");
        }
        _ => panic!("Expected CLIJSONDecode variant"),
    }

    assert_eq!(
        error.to_string(),
        "Failed to decode JSON response: Invalid JSON"
    );
}

#[test]
fn test_io_error_conversion() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let sdk_error: ClaudeSDKError = io_error.into();

    match sdk_error {
        ClaudeSDKError::Io(_) => {
            // Expected
        }
        _ => panic!("Expected Io variant"),
    }
}

#[test]
fn test_json_error_conversion() {
    let json_str = r#"{"invalid": json}"#;
    let json_error = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
    let sdk_error: ClaudeSDKError = json_error.into();

    match sdk_error {
        ClaudeSDKError::Json(_) => {
            // Expected
        }
        _ => panic!("Expected Json variant"),
    }
}

#[test]
fn test_error_display() {
    let errors = vec![
        ClaudeSDKError::CLINotFound,
        ClaudeSDKError::cli_connection("test"),
        ClaudeSDKError::process(1, "stderr"),
        ClaudeSDKError::cli_json_decode("json error"),
    ];

    for error in errors {
        // Ensure all errors implement Display properly
        let _display_string = error.to_string();

        // Ensure all errors implement Debug properly
        let _debug_string = format!("{:?}", error);
    }
}

#[test]
fn test_result_type() {
    let success: Result<i32> = Ok(42);
    let failure: Result<i32> = Err(ClaudeSDKError::CLINotFound);

    assert!(success.is_ok());
    assert!(failure.is_err());

    assert_eq!(success.unwrap(), 42);
    assert!(matches!(failure.unwrap_err(), ClaudeSDKError::CLINotFound));
}
