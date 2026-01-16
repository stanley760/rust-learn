use crate::error::ProcessError;
use crate::types::{OsType, ProcessRecord};
use std::process::Command;

/// NetStat Parser
/// 负责执行 netstat 命令并解析输出，提取进程网络连接信息
pub struct NetStatParser;

impl NetStatParser {
    /// 执行 netstat 命令并返回输出
    ///
    /// # Returns
    ///
    /// * `Result<String, ProcessError>` - 成功返回命令输出，失败返回错误
    pub fn execute_netstat() -> Result<String, ProcessError> {
        let os_type = OsType::current();
        let command_str = os_type.netstat_command();

        // 根据操作系统执行不同的命令
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", command_str]).output()
        } else {
            Command::new("sh").args(["-c", command_str]).output()
        };

        match output {
            Ok(output) => {
                if output.status.success() {
                    // 使用 from_utf8_lossy 来处理非 UTF-8 字符（如 Windows GBK 编码）
                    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                    Ok(output_str)
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(ProcessError::CommandFailed(format!(
                        "命令执行失败: {}",
                        stderr
                    )))
                }
            }
            Err(e) => Err(ProcessError::CommandFailed(format!("无法执行命令: {}", e))),
        }
    }

    /// 解析 netstat 命令输出
    ///
    /// # Arguments
    ///
    /// * `output` - netstat 命令的输出字符串
    ///
    /// # Returns
    ///
    /// * `Vec<ProcessRecord>` - 解析后的进程记录列表
    pub fn parse(output: &str) -> Vec<ProcessRecord> {
        let os_type = OsType::current();

        output
            .lines()
            .filter_map(|line| match os_type {
                OsType::Windows => Self::parse_windows(line),
                OsType::MacOS => Self::parse_macos(line),
                OsType::Linux => Self::parse_linux(line),
            })
            .collect()
    }

    /// 解析 Windows netstat -ano 输出格式
    ///
    /// 格式示例：
    /// ```
    /// TCP    0.0.0.0:135            0.0.0.0:0              LISTENING       1234
    /// TCP    192.168.1.100:50000    93.184.216.34:80       ESTABLISHED     5678
    /// ```
    ///
    /// # Arguments
    ///
    /// * `line` - 单行 netstat 输出
    ///
    /// # Returns
    ///
    /// * `Option<ProcessRecord>` - 成功解析返回 Some，失败返回 None
    fn parse_windows(line: &str) -> Option<ProcessRecord> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Windows netstat -ano 格式：协议 本地地址 远程地址 状态 PID
        // 至少需要 5 个字段
        if parts.len() < 5 {
            return None;
        }

        let protocol = parts[0];

        // 跳过标题行和非 TCP/UDP 行
        if protocol == "Proto"
            || protocol == "Active"
            || (!protocol.starts_with("TCP") && !protocol.starts_with("UDP"))
        {
            return None;
        }

        let local_address = parts[1];
        let remote_address = parts[2];
        let state = parts[3];
        let pid = parts[4];

        Some(ProcessRecord {
            protocol: protocol.to_string(),
            local_address: local_address.to_string(),
            remote_address: remote_address.to_string(),
            state: state.to_string(),
            pid: pid.to_string(),
        })
    }

    /// 解析 macOS netstat -anv 输出格式
    ///
    /// 格式示例：
    /// ```
    /// tcp4       0      0  192.168.1.100.50000    93.184.216.34.80       ESTABLISHED 131072 131768   5678      0 0x0102 0x00000020
    /// tcp4       0      0  *.8080                 *.*                    LISTEN      131072 131768   1234      0 0x0100 0x00000006
    /// ```
    ///
    /// # Arguments
    ///
    /// * `line` - 单行 netstat 输出
    ///
    /// # Returns
    ///
    /// * `Option<ProcessRecord>` - 成功解析返回 Some，失败返回 None
    fn parse_macos(line: &str) -> Option<ProcessRecord> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // macOS netstat -anv 格式较复杂，至少需要 9 个字段
        // 协议 Recv-Q Send-Q 本地地址 远程地址 状态 ... PID ...
        if parts.len() < 9 {
            return None;
        }

        let protocol = parts[0];

        // 跳过标题行和非 TCP/UDP 行
        if protocol == "Proto"
            || protocol == "Active"
            || (!protocol.starts_with("tcp") && !protocol.starts_with("udp"))
        {
            return None;
        }

        let local_address = parts[3];
        let remote_address = parts[4];
        let state = parts[5];

        // PID 通常在第 9 个位置（索引 8）
        let pid = if parts.len() > 8 { parts[8] } else { "0" };

        // 转换 macOS 地址格式（使用点号分隔）为标准格式（使用冒号分隔）
        let local_address = Self::convert_macos_address(local_address);
        let remote_address = Self::convert_macos_address(remote_address);

        Some(ProcessRecord {
            protocol: protocol.to_string(),
            local_address,
            remote_address,
            state: state.to_string(),
            pid: pid.to_string(),
        })
    }

    /// 解析 Linux netstat -antp 或 netstat -an 输出格式
    ///
    /// 格式示例（netstat -antp）：
    /// ```
    /// tcp        0      0 0.0.0.0:22              0.0.0.0:*               LISTEN      1234/sshd
    /// tcp        0      0 192.168.1.100:50000     93.184.216.34:80        ESTABLISHED 5678/firefox
    /// ```
    ///
    /// 格式示例（netstat -an，无 PID）：
    /// ```
    /// tcp        0      0 0.0.0.0:22              0.0.0.0:*               LISTEN
    /// ```
    ///
    /// # Arguments
    ///
    /// * `line` - 单行 netstat 输出
    ///
    /// # Returns
    ///
    /// * `Option<ProcessRecord>` - 成功解析返回 Some，失败返回 None
    fn parse_linux(line: &str) -> Option<ProcessRecord> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Linux netstat 格式：协议 Recv-Q Send-Q 本地地址 远程地址 状态 [PID/程序名]
        // 至少需要 6 个字段（状态字段）
        if parts.len() < 6 {
            return None;
        }

        let protocol = parts[0];

        // 跳过标题行和非 TCP/UDP 行
        if protocol == "Proto"
            || protocol == "Active"
            || (!protocol.starts_with("tcp") && !protocol.starts_with("udp"))
        {
            return None;
        }

        let local_address = parts[3];
        let remote_address = parts[4];
        let state = parts[5];

        // 提取 PID（如果存在）
        let pid = if parts.len() > 6 {
            // PID/程序名 格式，提取 PID 部分
            parts[6].split('/').next().unwrap_or("0")
        } else {
            "0"
        };

        Some(ProcessRecord {
            protocol: protocol.to_string(),
            local_address: local_address.to_string(),
            remote_address: remote_address.to_string(),
            state: state.to_string(),
            pid: pid.to_string(),
        })
    }

    /// 转换 macOS 地址格式
    ///
    /// macOS netstat 使用点号分隔地址和端口（例如：192.168.1.100.8080）
    /// 需要转换为标准格式（例如：192.168.1.100:8080）
    ///
    /// # Arguments
    ///
    /// * `address` - macOS 格式的地址字符串
    ///
    /// # Returns
    ///
    /// * `String` - 标准格式的地址字符串
    fn convert_macos_address(address: &str) -> String {
        // 处理通配符地址
        if address == "*.*" {
            return "*:*".to_string();
        }

        // 查找最后一个点号，将其替换为冒号
        if let Some(last_dot_pos) = address.rfind('.') {
            let mut result = address.to_string();
            result.replace_range(last_dot_pos..last_dot_pos + 1, ":");
            result
        } else {
            address.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_windows_valid_line() {
        let line = "TCP    0.0.0.0:135            0.0.0.0:0              LISTENING       1234";
        let result = NetStatParser::parse_windows(line);

        assert!(result.is_some());
        let record = result.unwrap();
        assert_eq!(record.protocol, "TCP");
        assert_eq!(record.local_address, "0.0.0.0:135");
        assert_eq!(record.remote_address, "0.0.0.0:0");
        assert_eq!(record.state, "LISTENING");
        assert_eq!(record.pid, "1234");
    }

    #[test]
    fn test_parse_windows_header_line() {
        let line = "Proto  Local Address          Foreign Address        State           PID";
        let result = NetStatParser::parse_windows(line);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_macos_valid_line() {
        let line = "tcp4       0      0  192.168.1.100.50000    93.184.216.34.80       ESTABLISHED 131072 131768   5678      0";
        let result = NetStatParser::parse_macos(line);

        assert!(result.is_some());
        let record = result.unwrap();
        assert_eq!(record.protocol, "tcp4");
        assert_eq!(record.local_address, "192.168.1.100:50000");
        assert_eq!(record.remote_address, "93.184.216.34:80");
        assert_eq!(record.state, "ESTABLISHED");
        assert_eq!(record.pid, "5678");
    }

    #[test]
    fn test_parse_linux_valid_line_with_pid() {
        let line = "tcp        0      0 0.0.0.0:22              0.0.0.0:*               LISTEN      1234/sshd";
        let result = NetStatParser::parse_linux(line);

        assert!(result.is_some());
        let record = result.unwrap();
        assert_eq!(record.protocol, "tcp");
        assert_eq!(record.local_address, "0.0.0.0:22");
        assert_eq!(record.remote_address, "0.0.0.0:*");
        assert_eq!(record.state, "LISTEN");
        assert_eq!(record.pid, "1234");
    }

    #[test]
    fn test_parse_linux_valid_line_without_pid() {
        let line = "tcp        0      0 0.0.0.0:22              0.0.0.0:*               LISTEN";
        let result = NetStatParser::parse_linux(line);

        assert!(result.is_some());
        let record = result.unwrap();
        assert_eq!(record.protocol, "tcp");
        assert_eq!(record.pid, "0");
    }

    #[test]
    fn test_convert_macos_address() {
        assert_eq!(
            NetStatParser::convert_macos_address("192.168.1.100.8080"),
            "192.168.1.100:8080"
        );
        assert_eq!(NetStatParser::convert_macos_address("*.*"), "*:*");
        assert_eq!(
            NetStatParser::convert_macos_address("127.0.0.1.3000"),
            "127.0.0.1:3000"
        );
    }

    #[test]
    fn test_parse_empty_output() {
        let output = "";
        let results = NetStatParser::parse(output);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_parse_multiple_lines() {
        let output = "Proto  Local Address          Foreign Address        State           PID\nTCP    0.0.0.0:135            0.0.0.0:0              LISTENING       1234\nTCP    192.168.1.100:50000    93.184.216.34:80       ESTABLISHED     5678";
        let results = NetStatParser::parse(output);

        // Should parse 2 valid lines (skipping header)
        assert_eq!(results.len(), 2);
    }
}
