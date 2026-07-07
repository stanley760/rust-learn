use std::borrow::Cow;
use std::collections::HashMap;

use async_openai::types::chat::{
    ChatCompletionTool, ChatCompletionTools, FunctionObject, FunctionObjectArgs,
};
use async_trait::async_trait;
use serde_json::Value;

mod bash;
mod write_file;
mod edit_file;
mod read_file;

/// Provider-agnostic tool specification.
///
/// Mirrors Anthropic's `ToolSpec` (name, description, input_schema)
/// and can be converted into any provider's native type.
pub struct ToolSpec {
    pub name: String,
    pub description: Option<String>,
    /// JSON Schema describing the tool's input parameters.
    pub input_schema: serde_json::Value,
}

impl ToolSpec {
    /// Convert into an OpenAI-compatible `ChatCompletionTools`.
    pub fn into_openai_tool(self) -> ChatCompletionTools {
        ChatCompletionTools::Function(ChatCompletionTool {
            function: self.into(),
        })
    }
}

impl From<ToolSpec> for FunctionObject {
    fn from(spec: ToolSpec) -> Self {
        let mut args = FunctionObjectArgs::default();
        args.name(spec.name);
        if let Some(desc) = spec.description {
            args.description(desc);
        }
        args.parameters(spec.input_schema);
        args.build().expect("Failed to build FunctionObject from ToolSpec")
    }
}

#[async_trait]
pub trait Tool: Send + Sync {
    async fn invoke(&self, input: &Value) -> anyhow::Result<String>;
    fn name(&self) -> Cow<'_, str>;
    fn tool_spec(&self) -> ToolSpec;
}

/// Return all registered tools as OpenAI-compatible `ChatCompletionTools`.
pub fn all_tools() -> Vec<ChatCompletionTools> {
    let tools: Vec<Box<dyn Tool>> = vec![
        Box::new(bash::BashTool),
        Box::new(edit_file::EditFileTool),
        Box::new(write_file::WriteFileTool), 
        Box::new(read_file::ReadFileTool),
    ];
    tools.into_iter().map(|t| t.tool_spec().into_openai_tool()).collect()
}

/// Return a name → tool registry for dispatching tool calls.
pub fn tool_registry() -> HashMap<String, Box<dyn Tool>> {
    let tools: Vec<Box<dyn Tool>> = vec![
        Box::new(bash::BashTool),
        Box::new(edit_file::EditFileTool),
        Box::new(write_file::WriteFileTool),
        Box::new(read_file::ReadFileTool),
    ];

    tools.into_iter().map(|t| {
        let name = t.name().into_owned();
        (name, t)
    }).collect()
}

/// Validate and resolve a path, ensuring it stays within the workspace.
///
/// If `must_exist` is true (e.g. for read operations), the path is canonicalized
/// which requires the file/directory to already exist.
/// If `must_exist` is false (e.g. for write operations), the parent directory
/// is canonicalized instead, and the filename is appended — this allows writing
/// to files that don't yet exist while still preventing path traversal.
fn safe_path(path: &str, must_exist: bool) -> anyhow::Result<std::path::PathBuf> {
    let cwd = std::env::current_dir()?;
    let full = cwd.join(path);

    if must_exist {
        let canonical = full.canonicalize()?;
        if !canonical.starts_with(&cwd) {
            return Err(anyhow::anyhow!("Path escapes workspace"));
        }
        Ok(canonical)
    } else {
        // For write operations: canonicalize the parent (which must exist),
        // then append the filename. This allows writing to new files while
        // still preventing path traversal attacks.
        let parent = full.parent().ok_or_else(|| anyhow::anyhow!("Path has no parent directory"))?;
        let canonical_parent = if parent.as_os_str().is_empty() {
            cwd.clone()
        } else {
            parent.canonicalize()?
        };
        if !canonical_parent.starts_with(&cwd) {
            return Err(anyhow::anyhow!("Path escapes workspace"));
        }
        let file_name = full.file_name().ok_or_else(|| anyhow::anyhow!("Path has no filename"))?;
        Ok(canonical_parent.join(file_name))
    }
}