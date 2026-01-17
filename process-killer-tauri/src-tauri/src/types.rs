use serde::{Deserialize, Serialize};

/// 进程记录数据结构
/// 包含网络连接的协议、地址、状态和进程 ID 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRecord {
    /// 网络协议（TCP、UDP 等）
    pub protocol: String,
    /// 本地地址（IP:Port 格式）
    pub local_address: String,
    /// 远程地址（IP:Port 格式）
    pub remote_address: String,
    /// 连接状态（LISTENING、ESTABLISHED 等）
    pub state: String,
    /// 进程 ID（字符串格式，便于显示）
    pub pid: String,
}

/// 分页请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    /// 当前页码（从 1 开始）
    pub page: usize,
    /// 每页数量
    pub page_size: usize,
}

/// 分页响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 当前页数据
    pub data: Vec<T>,
    /// 当前页码
    pub page: usize,
    /// 每页数量
    pub page_size: usize,
    /// 总记录数
    pub total: usize,
    /// 总页数
    pub total_pages: usize,
}

/// 操作系统类型枚举
/// 用于识别当前操作系统并提供对应的 netstat 命令
#[derive(Debug, Clone, Copy)]
pub enum OsType {
    Windows,
    MacOS,
    Linux,
}

impl OsType {
    /// 检测当前操作系统类型
    pub fn current() -> Self {
        if cfg!(target_os = "windows") {
            OsType::Windows
        } else if cfg!(target_os = "macos") {
            OsType::MacOS
        } else {
            OsType::Linux
        }
    }

    /// 返回对应操作系统的 netstat 命令
    pub fn netstat_command(&self) -> &str {
        match self {
            OsType::Windows => "netstat -ano",
            OsType::MacOS => "netstat -anv",
            OsType::Linux => "netstat -antp 2>/dev/null || netstat -an",
        }
    }
}
