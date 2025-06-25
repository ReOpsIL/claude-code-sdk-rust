use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PermissionMode {
    Default,
    AcceptEdits,
    BypassPermissions,
}

impl Default for PermissionMode {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub command: String,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    pub text: String,
}

impl TextBlock {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self {
            block_type: "text".to_string(),
            text: text.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUseBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    pub id: String,
    pub name: String,
    pub input: serde_json::Value,
}

impl ToolUseBlock {
    pub fn new<S: Into<String>>(id: S, name: S, input: serde_json::Value) -> Self {
        Self {
            block_type: "tool_use".to_string(),
            id: id.into(),
            name: name.into(),
            input,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResultBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    pub tool_use_id: String,
    pub content: Option<String>,
    pub is_error: Option<bool>,
}

impl ToolResultBlock {
    pub fn new<S: Into<String>>(
        tool_use_id: S,
        content: Option<S>,
        is_error: Option<bool>,
    ) -> Self {
        Self {
            block_type: "tool_result".to_string(),
            tool_use_id: tool_use_id.into(),
            content: content.map(|c| c.into()),
            is_error,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentBlock {
    Text(TextBlock),
    ToolUse(ToolUseBlock),
    ToolResult(ToolResultBlock),
}

impl From<TextBlock> for ContentBlock {
    fn from(block: TextBlock) -> Self {
        Self::Text(block)
    }
}

impl From<ToolUseBlock> for ContentBlock {
    fn from(block: ToolUseBlock) -> Self {
        Self::ToolUse(block)
    }
}

impl From<ToolResultBlock> for ContentBlock {
    fn from(block: ToolResultBlock) -> Self {
        Self::ToolResult(block)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub content: Vec<ContentBlock>,
}

impl UserMessage {
    pub fn new(content: Vec<ContentBlock>) -> Self {
        Self {
            message_type: "user".to_string(),
            content,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub content: Vec<ContentBlock>,
}

impl AssistantMessage {
    pub fn new(content: Vec<ContentBlock>) -> Self {
        Self {
            message_type: "assistant".to_string(),
            content,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub content: String,
}

impl SystemMessage {
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            message_type: "system".to_string(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub id: String,
    pub exit_code: Option<i32>,
    pub content: Option<String>,
    pub cost_usd: Option<f64>,
    pub tokens_input: Option<i32>,
    pub tokens_output: Option<i32>,
    pub reasoning_tokens: Option<i32>,
    pub canceled: Option<bool>,
}

impl ResultMessage {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self {
            message_type: "result".to_string(),
            id: id.into(),
            exit_code: None,
            content: None,
            cost_usd: None,
            tokens_input: None,
            tokens_output: None,
            reasoning_tokens: None,
            canceled: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    User(UserMessage),
    Assistant(AssistantMessage),
    System(SystemMessage),
    Result(ResultMessage),
}

impl From<UserMessage> for Message {
    fn from(msg: UserMessage) -> Self {
        Self::User(msg)
    }
}

impl From<AssistantMessage> for Message {
    fn from(msg: AssistantMessage) -> Self {
        Self::Assistant(msg)
    }
}

impl From<SystemMessage> for Message {
    fn from(msg: SystemMessage) -> Self {
        Self::System(msg)
    }
}

impl From<ResultMessage> for Message {
    fn from(msg: ResultMessage) -> Self {
        Self::Result(msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodeOptions {
    pub cwd: Option<PathBuf>,
    pub allowed_tools: Option<Vec<String>>,
    pub permission_mode: Option<PermissionMode>,
    pub system_prompt: Option<String>,
    pub max_turns: Option<i32>,
    pub disable_safety_suggestions: Option<bool>,
    pub disable_telemetry: Option<bool>,
    pub disable_stream: Option<bool>,
    pub disable_vision: Option<bool>,
    pub disable_search: Option<bool>,
    pub claude_model: Option<String>,
    pub claude_host: Option<String>,
    pub claude_api_key: Option<String>,
    pub claude_anthropic_version: Option<String>,
    pub claude_max_tokens: Option<i32>,
    pub claude_temperature: Option<f64>,
    pub claude_top_k: Option<i32>,
    pub claude_top_p: Option<f64>,
    pub claude_stop_sequences: Option<Vec<String>>,
    pub claude_timeout: Option<i32>,
    pub claude_stream: Option<bool>,
    pub claude_extra_headers: Option<HashMap<String, String>>,
    pub claude_default_headers: Option<HashMap<String, String>>,
    pub mcp_servers: Option<Vec<McpServerConfig>>,
    pub mcp_timeout: Option<i32>,
    pub mcp_disable_tools: Option<bool>,
    pub mcp_disable_resources: Option<bool>,
    pub mcp_disable_prompts: Option<bool>,
    pub mcp_disable_sampling: Option<bool>,
    pub mcp_disable_roots: Option<bool>,
    pub mcp_extra_logging: Option<bool>,
    pub mcp_batch_requests: Option<bool>,
    pub mcp_batch_delay: Option<i32>,
    pub allow_tools: Option<bool>,
    pub no_tools: Option<bool>,
    pub no_prompt_validation: Option<bool>,
    pub no_prompt_cache: Option<bool>,
    pub no_model_timeout: Option<bool>,
    pub no_output_timeout: Option<bool>,
    pub no_input_timeout: Option<bool>,
    pub input_timeout: Option<i32>,
    pub output_timeout: Option<i32>,
    pub model_timeout: Option<i32>,
    pub prompt_cache_dir: Option<PathBuf>,
    pub log_level: Option<String>,
    pub config_file: Option<PathBuf>,
    pub env: Option<HashMap<String, String>>,
}

impl Default for ClaudeCodeOptions {
    fn default() -> Self {
        Self {
            cwd: None,
            allowed_tools: None,
            permission_mode: None,
            system_prompt: None,
            max_turns: None,
            disable_safety_suggestions: None,
            disable_telemetry: None,
            disable_stream: None,
            disable_vision: None,
            disable_search: None,
            claude_model: None,
            claude_host: None,
            claude_api_key: None,
            claude_anthropic_version: None,
            claude_max_tokens: None,
            claude_temperature: None,
            claude_top_k: None,
            claude_top_p: None,
            claude_stop_sequences: None,
            claude_timeout: None,
            claude_stream: None,
            claude_extra_headers: None,
            claude_default_headers: None,
            mcp_servers: None,
            mcp_timeout: None,
            mcp_disable_tools: None,
            mcp_disable_resources: None,
            mcp_disable_prompts: None,
            mcp_disable_sampling: None,
            mcp_disable_roots: None,
            mcp_extra_logging: None,
            mcp_batch_requests: None,
            mcp_batch_delay: None,
            allow_tools: None,
            no_tools: None,
            no_prompt_validation: None,
            no_prompt_cache: None,
            no_model_timeout: None,
            no_output_timeout: None,
            no_input_timeout: None,
            input_timeout: None,
            output_timeout: None,
            model_timeout: None,
            prompt_cache_dir: None,
            log_level: None,
            config_file: None,
            env: None,
        }
    }
}

impl ClaudeCodeOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cwd<P: Into<PathBuf>>(mut self, cwd: P) -> Self {
        self.cwd = Some(cwd.into());
        self
    }

    pub fn with_allowed_tools(mut self, tools: Vec<String>) -> Self {
        self.allowed_tools = Some(tools);
        self
    }

    pub fn with_permission_mode(mut self, mode: PermissionMode) -> Self {
        self.permission_mode = Some(mode);
        self
    }

    pub fn with_system_prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    pub fn with_max_turns(mut self, turns: i32) -> Self {
        self.max_turns = Some(turns);
        self
    }
}
