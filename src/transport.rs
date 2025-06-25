use crate::error::{ClaudeSDKError, Result};
use crate::types::{ClaudeCodeOptions, Message, PermissionMode};
use async_trait::async_trait;
use futures::stream::Stream;
use serde_json;
use std::path::PathBuf;
use std::pin::Pin;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;
use which::which;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn connect(&mut self) -> Result<()>;
    async fn disconnect(&mut self) -> Result<()>;
    async fn receive_messages(
        &mut self,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Message>> + Send>>>;
    fn is_connected(&self) -> bool;
}

pub struct SubprocessCLITransport {
    child: Option<Child>,
    connected: bool,
    options: ClaudeCodeOptions,
    prompt: String,
}

impl SubprocessCLITransport {
    pub fn new(prompt: String, options: ClaudeCodeOptions) -> Self {
        Self {
            child: None,
            connected: false,
            options,
            prompt,
        }
    }

    fn find_cli_binary() -> Result<PathBuf> {
        // Common installation paths for Claude Code CLI
        let paths = [
            "claude-code",
            "/usr/local/bin/claude-code",
            "/opt/homebrew/bin/claude-code",
            // Add more paths as needed based on common installation locations
        ];

        for path in &paths {
            if let Ok(binary_path) = which(path) {
                return Ok(binary_path);
            }
        }

        Err(ClaudeSDKError::CLINotFound)
    }

    fn build_command(&self) -> Result<Command> {
        let binary_path = Self::find_cli_binary()?;
        let mut cmd = Command::new(binary_path);

        // Set working directory
        if let Some(cwd) = &self.options.cwd {
            cmd.current_dir(cwd);
        }

        // Build CLI arguments based on options
        cmd.arg("--format").arg("json");

        if let Some(system_prompt) = &self.options.system_prompt {
            cmd.arg("--system").arg(system_prompt);
        }

        if let Some(max_turns) = self.options.max_turns {
            cmd.arg("--max-turns").arg(max_turns.to_string());
        }

        if let Some(permission_mode) = &self.options.permission_mode {
            match permission_mode {
                PermissionMode::AcceptEdits => {
                    cmd.arg("--accept-edits");
                }
                PermissionMode::BypassPermissions => {
                    cmd.arg("--bypass-permissions");
                }
                PermissionMode::Default => {
                    // No additional flags needed for default mode
                }
            }
        }

        if let Some(allowed_tools) = &self.options.allowed_tools {
            for tool in allowed_tools {
                cmd.arg("--tool").arg(tool);
            }
        }

        if self.options.disable_safety_suggestions.unwrap_or(false) {
            cmd.arg("--disable-safety-suggestions");
        }

        if self.options.disable_telemetry.unwrap_or(false) {
            cmd.arg("--disable-telemetry");
        }

        if self.options.disable_stream.unwrap_or(false) {
            cmd.arg("--disable-stream");
        }

        if self.options.disable_vision.unwrap_or(false) {
            cmd.arg("--disable-vision");
        }

        if self.options.disable_search.unwrap_or(false) {
            cmd.arg("--disable-search");
        }

        if let Some(claude_model) = &self.options.claude_model {
            cmd.arg("--model").arg(claude_model);
        }

        if let Some(claude_api_key) = &self.options.claude_api_key {
            cmd.env("ANTHROPIC_API_KEY", claude_api_key);
        }

        if let Some(env_vars) = &self.options.env {
            for (key, value) in env_vars {
                cmd.env(key, value);
            }
        }

        // Add the prompt as the final argument
        cmd.arg(&self.prompt);

        // Configure stdio
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null());

        Ok(cmd)
    }
}

#[async_trait]
impl Transport for SubprocessCLITransport {
    async fn connect(&mut self) -> Result<()> {
        if self.connected {
            return Ok(());
        }

        let mut cmd = self.build_command()?;
        let child = cmd.spawn().map_err(|e| {
            ClaudeSDKError::cli_connection(format!("Failed to spawn CLI process: {}", e))
        })?;

        self.child = Some(child);
        self.connected = true;
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            // Attempt graceful shutdown first
            if let Err(_) = child.kill().await {
                // If graceful shutdown fails, force kill
                let _ = child.wait().await;
            }
        }
        self.connected = false;
        Ok(())
    }

    async fn receive_messages(
        &mut self,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Message>> + Send>>> {
        if !self.connected {
            return Err(ClaudeSDKError::cli_connection("Not connected"));
        }

        let child = self
            .child
            .as_mut()
            .ok_or_else(|| ClaudeSDKError::cli_connection("No child process available"))?;

        let stdout = child.stdout.take().ok_or_else(|| {
            ClaudeSDKError::cli_connection("Failed to get stdout from child process")
        })?;

        let reader = BufReader::new(stdout);
        let lines_stream = LinesStream::new(reader.lines());

        let message_stream = lines_stream.map(|line_result| {
            let line = line_result.map_err(|e| ClaudeSDKError::Io(e))?;

            if line.trim().is_empty() {
                return Err(ClaudeSDKError::cli_json_decode("Empty line received"));
            }

            let message: Message = serde_json::from_str(&line).map_err(|e| {
                ClaudeSDKError::cli_json_decode(format!("Failed to parse JSON: {}", e))
            })?;

            Ok(message)
        });

        Ok(Box::pin(message_stream))
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}

impl Drop for SubprocessCLITransport {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            // Best effort cleanup
            let _ = child.start_kill();
        }
    }
}
