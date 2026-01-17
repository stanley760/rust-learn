use super::{NetStatParser, SysInfoWrapper};
use crate::error::ProcessError;
use crate::types::{PageRequest, PageResponse, ProcessRecord};

/// Process Manager
/// 核心业务逻辑模块，封装进程管理功能
pub struct ProcessManager {
    sysinfo_wrapper: SysInfoWrapper,
}

impl ProcessManager {
    /// 创建新的 ProcessManager 实例
    pub fn new() -> Self {
        Self {
            sysinfo_wrapper: SysInfoWrapper::new(),
        }
    }

    /// 获取所有网络连接进程
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<ProcessRecord>)` - 成功返回进程列表
    /// * `Err(ProcessError)` - 获取失败返回错误
    pub fn get_all_processes(&mut self) -> Result<Vec<ProcessRecord>, ProcessError> {
        // 执行 netstat 命令获取输出
        let output = NetStatParser::execute_netstat()?;

        // 解析输出并返回进程列表
        let processes = NetStatParser::parse(&output);

        Ok(processes)
    }

    /// 获取所有网络连接进程（分页）
    ///
    /// # Arguments
    ///
    /// * `request` - 分页请求参数
    ///
    /// # Returns
    ///
    /// * `Ok(PageResponse<ProcessRecord>)` - 成功返回分页数据
    /// * `Err(ProcessError)` - 获取失败返回错误
    pub fn get_all_processes_paginated(
        &mut self,
        request: &PageRequest,
    ) -> Result<PageResponse<ProcessRecord>, ProcessError> {
        // 获取所有进程
        let all_processes = self.get_all_processes()?;

        // 应用分页
        Ok(Self::paginate(all_processes, request))
    }

    /// 按端口号搜索进程
    ///
    /// # Arguments
    ///
    /// * `port` - 端口号字符串
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<ProcessRecord>)` - 成功返回匹配的进程列表
    /// * `Err(ProcessError)` - 搜索失败返回错误
    pub fn search_by_port(&mut self, port: &str) -> Result<Vec<ProcessRecord>, ProcessError> {
        // 验证端口输入非空
        if port.trim().is_empty() {
            return Err(ProcessError::EmptyPort);
        }

        // 验证端口输入为数字
        let port_num = port
            .trim()
            .parse::<u32>()
            .map_err(|_| ProcessError::InvalidPort)?;

        // 获取所有进程
        let all_processes = self.get_all_processes()?;

        // 过滤包含指定端口的进程
        let filtered_processes: Vec<ProcessRecord> = all_processes
            .into_iter()
            .filter(|process| Self::extract_port(&process.local_address) == Some(port_num))
            .collect();

        // 如果没有找到匹配的进程，返回错误
        if filtered_processes.is_empty() {
            return Err(ProcessError::PortNotFound(port_num));
        }

        Ok(filtered_processes)
    }

    /// 按端口号搜索进程（分页）
    ///
    /// # Arguments
    ///
    /// * `port` - 端口号字符串
    /// * `request` - 分页请求参数
    ///
    /// # Returns
    ///
    /// * `Ok(PageResponse<ProcessRecord>)` - 成功返回分页数据
    /// * `Err(ProcessError)` - 搜索失败返回错误
    pub fn search_by_port_paginated(
        &mut self,
        port: &str,
        request: &PageRequest,
    ) -> Result<PageResponse<ProcessRecord>, ProcessError> {
        // 验证端口输入非空
        if port.trim().is_empty() {
            return Err(ProcessError::EmptyPort);
        }

        // 验证端口输入为数字
        let port_num = port
            .trim()
            .parse::<u32>()
            .map_err(|_| ProcessError::InvalidPort)?;

        // 获取所有进程
        let all_processes = self.get_all_processes()?;

        // 过滤包含指定端口的进程
        let filtered_processes: Vec<ProcessRecord> = all_processes
            .into_iter()
            .filter(|process| Self::extract_port(&process.local_address) == Some(port_num))
            .collect();

        // 如果没有找到匹配的进程，返回空分页结果
        if filtered_processes.is_empty() {
            return Ok(PageResponse {
                data: vec![],
                page: request.page,
                page_size: request.page_size,
                total: 0,
                total_pages: 0,
            });
        }

        // 应用分页
        Ok(Self::paginate(filtered_processes, request))
    }

    /// 从地址字符串中提取端口号
    ///
    /// 支持 IPv4 格式（IP:Port）和 IPv6 格式（[IP]:Port）
    ///
    /// # Arguments
    ///
    /// * `address` - 地址字符串
    ///
    /// # Returns
    ///
    /// * `Some(u32)` - 成功提取端口号
    /// * `None` - 提取失败
    fn extract_port(address: &str) -> Option<u32> {
        // 处理 IPv6 格式：[IP]:Port
        if address.contains('[') {
            // 查找最后一个冒号（端口分隔符）
            if let Some(colon_pos) = address.rfind("]:") {
                let port_str = &address[colon_pos + 2..];
                return port_str.parse::<u32>().ok();
            }
        }

        // 处理 IPv4 格式：IP:Port
        // 查找最后一个冒号
        if let Some(colon_pos) = address.rfind(':') {
            let port_str = &address[colon_pos + 1..];
            return port_str.parse::<u32>().ok();
        }

        None
    }

    /// 对数据进行分页处理
    ///
    /// # Arguments
    ///
    /// * `data` - 原始数据列表
    /// * `request` - 分页请求参数
    ///
    /// # Returns
    ///
    /// * `PageResponse<ProcessRecord>` - 分页响应数据
    fn paginate(data: Vec<ProcessRecord>, request: &PageRequest) -> PageResponse<ProcessRecord> {
        let total = data.len();
        let page_size = request.page_size.max(1); // 确保 page_size 至少为 1
        let total_pages = (total + page_size - 1) / page_size; // 向上取整

        // 确保页码在有效范围内
        let page = if request.page < 1 {
            1
        } else if request.page > total_pages && total_pages > 0 {
            total_pages
        } else {
            request.page
        };

        // 计算起始和结束索引
        let start = (page - 1) * page_size;
        let end = (start + page_size).min(total);

        // 提取当前页数据
        let page_data = if start < total {
            data[start..end].to_vec()
        } else {
            vec![]
        };

        PageResponse {
            data: page_data,
            page,
            page_size,
            total,
            total_pages,
        }
    }

    /// 终止指定 PID 的进程
    ///
    /// # Arguments
    ///
    /// * `pid` - 进程 ID 字符串
    ///
    /// # Returns
    ///
    /// * `Ok(())` - 进程成功终止
    /// * `Err(ProcessError)` - 终止失败返回错误
    pub fn kill_process(&mut self, pid: &str) -> Result<(), ProcessError> {
        // 验证 PID 输入非空
        if pid.trim().is_empty() {
            return Err(ProcessError::EmptyPid);
        }

        // 验证 PID 输入为数字
        let pid_num = pid
            .trim()
            .parse::<u32>()
            .map_err(|_| ProcessError::InvalidPid)?;

        // 调用 SysInfoWrapper 终止进程
        self.sysinfo_wrapper.kill_process(pid_num)
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_port_ipv4() {
        // Test IPv4 format: IP:Port
        assert_eq!(
            ProcessManager::extract_port("192.168.1.100:8080"),
            Some(8080)
        );
        assert_eq!(ProcessManager::extract_port("0.0.0.0:80"), Some(80));
        assert_eq!(ProcessManager::extract_port("127.0.0.1:3000"), Some(3000));
    }

    #[test]
    fn test_extract_port_ipv6() {
        // Test IPv6 format: [IP]:Port
        assert_eq!(ProcessManager::extract_port("[::1]:8080"), Some(8080));
        assert_eq!(ProcessManager::extract_port("[2001:db8::1]:443"), Some(443));
        assert_eq!(ProcessManager::extract_port("[fe80::1]:3000"), Some(3000));
    }

    #[test]
    fn test_extract_port_invalid() {
        // Test invalid formats
        assert_eq!(ProcessManager::extract_port("invalid"), None);
        assert_eq!(ProcessManager::extract_port("192.168.1.100"), None);
        assert_eq!(ProcessManager::extract_port(""), None);
    }

    #[test]
    fn test_search_by_port_empty_input() {
        let mut manager = ProcessManager::new();
        let result = manager.search_by_port("");

        assert!(result.is_err());
        match result {
            Err(ProcessError::EmptyPort) => (),
            _ => panic!("Expected EmptyPort error"),
        }
    }

    #[test]
    fn test_search_by_port_invalid_format() {
        let mut manager = ProcessManager::new();
        let result = manager.search_by_port("abc");

        assert!(result.is_err());
        match result {
            Err(ProcessError::InvalidPort) => (),
            _ => panic!("Expected InvalidPort error"),
        }
    }

    #[test]
    fn test_kill_process_empty_input() {
        let mut manager = ProcessManager::new();
        let result = manager.kill_process("");

        assert!(result.is_err());
        match result {
            Err(ProcessError::EmptyPid) => (),
            _ => panic!("Expected EmptyPid error"),
        }
    }

    #[test]
    fn test_kill_process_invalid_format() {
        let mut manager = ProcessManager::new();
        let result = manager.kill_process("abc");

        assert!(result.is_err());
        match result {
            Err(ProcessError::InvalidPid) => (),
            _ => panic!("Expected InvalidPid error"),
        }
    }
}
