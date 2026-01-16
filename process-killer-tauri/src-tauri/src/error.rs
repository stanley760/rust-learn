use thiserror::Error;

/// 进程管理错误类型
/// 定义所有可能的错误情况，提供用户友好的错误消息
#[derive(Debug, Error)]
pub enum ProcessError {
    /// 端口号不能为空
    #[error("端口号不能为空")]
    EmptyPort,

    /// 端口号必须是数字
    #[error("端口号必须是数字")]
    InvalidPort,

    /// 未找到使用指定端口的进程
    #[error("未找到使用端口 {0} 的进程")]
    PortNotFound(u32),

    /// PID 不能为空
    #[error("PID 不能为空")]
    EmptyPid,

    /// PID 必须是数字
    #[error("PID 必须是数字")]
    InvalidPid,

    /// 未找到指定 PID 的进程
    #[error("未找到 PID 为 {0} 的进程")]
    ProcessNotFound(u32),

    /// 无法终止进程（可能需要管理员权限）
    #[error("无法终止进程 {0} (可能需要管理员权限)")]
    KillFailed(u32),

    /// 无法获取进程列表
    #[error("无法获取进程列表: {0}")]
    CommandFailed(String),
}

/// 将 ProcessError 转换为 String，用于传递给前端
impl From<ProcessError> for String {
    fn from(err: ProcessError) -> String {
        err.to_string()
    }
}
