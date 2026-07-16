pub const READ_ONLY_TOOLS: &[&str] = &["read_file", "bash_readonly"];
pub const WRITE_TOOLS: &[&str] = &["write_file", "edit_file", "bash"];

pub(crate) fn is_read_only_tool(tool_name: &str) -> bool {
    READ_ONLY_TOOLS.contains(&tool_name)
}

pub(crate) fn is_write_tool(tool_name: &str) -> bool {
    WRITE_TOOLS.contains(&tool_name)
}
