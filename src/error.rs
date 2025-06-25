use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClaudeSDKError {
    #[error("CLI connection error: {message}")]
    CLIConnection { message: String },

    #[error("Claude Code CLI not found. Please install it with: npm install -g @anthropic-ai/claude-code")]
    CLINotFound,

    #[error("Process failed with exit code {exit_code}: {stderr}")]
    Process { exit_code: i32, stderr: String },

    #[error("Failed to decode JSON response: {message}")]
    CLIJSONDecode { message: String },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Timeout error: {0}")]
    Timeout(#[from] tokio::time::error::Elapsed),

    #[error("Binary discovery error: {0}")]
    Which(#[from] which::Error),
}

pub type Result<T> = std::result::Result<T, ClaudeSDKError>;

impl ClaudeSDKError {
    pub fn cli_connection<S: Into<String>>(message: S) -> Self {
        Self::CLIConnection {
            message: message.into(),
        }
    }

    pub fn process<S: Into<String>>(exit_code: i32, stderr: S) -> Self {
        Self::Process {
            exit_code,
            stderr: stderr.into(),
        }
    }

    pub fn cli_json_decode<S: Into<String>>(message: S) -> Self {
        Self::CLIJSONDecode {
            message: message.into(),
        }
    }
}
